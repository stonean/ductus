---
section: "Bash script relationships"
---

# Derive-references-unstaged-drift-is-reported

## Context

An adopter renamed a service alias in `.ductus/config.toml` — `[services.api]`
became `[services.svc-zmc-api]`, same `repo` URL. Two of their specs carried
`references:` entries naming the old alias. Those entries were dead for **nine
commits**, and the pre-commit hook reported the tree in sync on every one of
them.

Reproduced on runtime 0.33.0, in a scratch repo, on identical state — one spec
drifted on both derived axes and deliberately left unstaged:

```text
derive-dependencies --staged  {"drift":false,"updated":[],"unwritten":["specs/001-alerts/spec.md"],"examined":2,…}
derive-references   --staged  {"drift":false,"updated":[],"examined":1,"untracked-skipped":[],"unparseable":[],…}
```

Every honesty field on the references payload reads clean. `drift` is false,
`updated` is empty, `untracked-skipped` is empty, `unparseable` is empty.
Nothing in that result lets a caller learn that a tracked spec is carrying a
reference to a service alias that no longer exists.

The two primitives diverge at enumeration.
[`derive_dependencies`](../../../runtime/src/primitives/derive_dependencies.rs)
walks **every tracked spec**, derives, and then consults the staged set only to
decide whether to *write* — a spec that was examined and found drifted but left
alone lands in `unwritten`.
[`derive_references`](../../../runtime/src/primitives/derive_references.rs)
narrows the **enumeration** to the staged set, so a tracked-but-unstaged
drifted spec is never examined at all, and `DeriveReferencesResult` has no
field that could hold it.

### The recorded rationale is falsified

Two artifacts justify the split, and both are wrong:

> Each spec's `references:` is a pure function of its own body — there is no
> cross-spec graph here, unlike the dependency cycle check. So `--staged`
> narrows the enumeration itself rather than filtering a full walk.

and, in
[017's `generator-sync-claim-honesty`](../../017-derive-dont-ask/scenarios/generator-sync-claim-honesty.md):

> This case is specific to `gen-spec-deps.sh`: `gen-cross-service-refs.sh`
> writes every spec it enumerates, so for it "enumerated" and "written" are the
> same set and its claim was already sound.

**`references:` is not a pure function of the spec body.** It is a function of
the body *and* the `[services]` registry — `harvest` resolves each link's repo
URL through the registry to produce the `service:` alias. That is the asymmetry
the rationale misses:

- A **dependency** derives from the body alone, so it can only drift when its
  own spec is edited — which stages that spec, which makes it visible.
- A **reference** drifts when the *config* changes and the spec is untouched.
  An untouched spec is never staged, so staged mode can never examine it, on
  any commit, for any number of commits.

So the conclusion inverts: `derive-references` needs the full walk **more**
than `derive-dependencies` does, not less. The "enumerated equals written"
reasoning holds only for a body-derived field, and this is the one derived
field that has an input outside the body.

Nothing else covers the gap. `check-orphaned-references` has a different
subject entirely — filesystem paths in four adopter-owned files — and does not
look at frontmatter aliases.

This is `QUAL-CLAIM-001` in the same shape the sibling scenario
[derive-unparseable-frontmatter-is-reported](derive-unparseable-frontmatter-is-reported.md)
closed, one category over: there the subject was reachable and went unexamined,
here it is tracked and goes unenumerated. Both emit the payload of a clean run.

## Behavior

`derive-references` enumerates **every tracked spec**, exactly as
`derive-dependencies` does, and consults the staged set only to decide whether
to write.

`DeriveReferencesResult` gains `unwritten: Vec<String>` — the repo-relative
paths of specs examined and found drifted but deliberately not written, sorted.
The field carries the meaning it already has on `DeriveDependenciesResult`:
neither "in sync" nor "not examined", so reported as neither.

`examined` consequently counts every tracked spec rather than the staged
subset, which is what makes the count comparable between the two primitives on
the same tree.

The write rule is unchanged: under `--staged`, only staged specs are rewritten,
so committing one spec still never rewrites the derived frontmatter of an
unrelated one. This is a reporting change, not a write-behavior change — the
same distinction 017's scenario drew for `gen-spec-deps.sh`.

The falsified paragraph in
[017's `generator-sync-claim-honesty`](../../017-derive-dont-ask/scenarios/generator-sync-claim-honesty.md)
is corrected rather than left standing, and the `QUAL-CLAIM-001` Source note in
`framework/rules/quality-cross.md` — which records both derivations as already
reporting `unwritten` — is corrected with it. A scenario that still asserts the
reasoning this one overturns would leave two competing authorities in the
corpus, which is the drift the canonical-source map exists to prevent.

## Edge Cases

- **A registry rename with no spec staged at all.** The commit touches only
  `.ductus/config.toml`. Every drifted spec lands in `unwritten`; nothing is
  written; the hook does not block. The operator now has the list, which is the
  entire point — nine commits of silence is what the absent list cost.
- **A service removed from the registry** drifts every referencing spec to
  `service: null` by the same mechanism, and is reported the same way.
- **Unregistered-repo references are unaffected.** They already resolve to
  `service: null` regardless of the registry, so no registry edit can drift
  them.
- **Full-walk mode is unchanged.** Without `--staged` every tracked spec is
  both enumerated and written, so `unwritten` is always empty there — the same
  invariant `derive-dependencies` has.
- **`--staged` scoping still bounds `unparseable`,** whose own scenario notes
  that an unparseable spec outside the staged set cannot appear. That note is
  written against the old enumeration and no longer holds for this primitive:
  with the full walk, an unparseable tracked spec is reported whether or not it
  is staged. The sibling scenario's edge case is updated to say so.
- **The field is additive.** Existing consumers ignore it and the MCP goldens
  that assert the ordinary payload stay byte-identical when it is empty.
- **`data-model.md` records it.** That file currently scopes `unwritten` to
  `derive-dependencies` only; the annotation is removed, since the field now
  belongs to both result shapes and a markdown-only host reads that file to
  know what to expect.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
