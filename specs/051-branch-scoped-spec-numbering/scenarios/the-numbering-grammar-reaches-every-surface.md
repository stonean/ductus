---
section: "Interaction with existing surfaces"
---

# The-numbering-grammar-reaches-every-surface

## Context

This feature's governing constraint is that the directory-membership rule lives in exactly one place: `parse_feature_dir`, which `is_feature_slug`, `list_feature_dirs` and `is_spec_path` all delegate to, so widening the corpus happened once rather than once per consumer (AC15).

That holds for the runtime. It does not hold for the repository, because five shell surfaces carry their own copy of the three-digit rule and the single predicate never reached them:

| Surface | Consequence for a spec numbered past 999 |
| --- | --- |
| `.githooks/pre-commit` | selects staged specs with `[0-9][0-9][0-9]-`, so `label-criteria` never runs on it — spec 013's labelling backstop is dead for that spec |
| `framework/bootstrap/hooks/ductus-pre-commit` | the same defect in the copy **adopters** get, which is a separate file |
| `scripts/lint-frontmatter.sh` | its `spec.md`, `spec-and-plan.md` and scenarios are never frontmatter-linted |
| `scripts/audit/sibling-coupling.sh` | audit Family 12 skips the directory |
| `scripts/audit/introducing-drift.sh` | that family skips it too |

The three lint and audit surfaces fail **silently and green**: they do not error, they simply never see the directory, which is [§design-principles](../../../framework/constitution.md#design-principles)' *a check that cannot run must never look like one that passed*.

Nothing is broken today — this repository is at spec 051 and no adopter is near 1000. What changed is that widening `parse_sequential` made the runtime willing to *create* the directory that triggers it, so the gap arrived with the fix rather than existing before it.

## Behavior

A spec directory the runtime recognizes is a spec directory to every surface of the project, shell included. Whatever the recognizer accepts, the pre-commit hooks stage, the frontmatter lint reads, and the audit families examine.

The rule has one statement and the shell asks for it rather than restating it. Where a surface can call the runtime, it calls the runtime — `scripts/audit/command-flag-hint-parity.sh` is the reference shape, an entry point in shell with the logic in a primitive ([§runtime-boundary](../../../framework/constitution.md#runtime-boundary)). Where a surface genuinely cannot — a `git diff --cached` filter inside a hook that must run before any binary is resolved — the pattern it uses accepts what the grammar accepts, and its agreement with the runtime is covered by a check rather than by a reader's memory.

Both hook copies change together. They are separate files, and a fix applied only to the one this repository executes leaves every adopter broken while the local run stays green.

## Edge Cases

- **The hook runs before the runtime is resolvable.** The hook's spec-matching happens on every commit, including in a tree where `.ductus/bin/ductus` is absent, so that one filter stays a pattern rather than a call. Widening it to three-or-more digits is the whole change there; the pattern must still reject a name the runtime rejects, so padding past three digits (`0500-`) is not admitted.
- **A four-digit directory that the runtime rejects** — `0500-a` — must be skipped by the shell surfaces too, or the two disagree in the opposite direction and the shell lints a directory no corpus reader can see.
- **An adopter whose spec root is renamed** already exercises the hook's shape-matching (audit Family 22), so the digit change must not disturb the leading-segment wildcard that feature depends on.
- **The audit families run without a runtime.** Family checks resolve the binary and report a precondition failure when it is missing, so a family that starts asking the runtime for the corpus inherits that behavior rather than silently examining nothing.
- **A corpus with no such directory** — every corpus today — must show no behavior change at all, which is what makes this safe to land ahead of the case that needs it.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
