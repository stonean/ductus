# 052 — Spec supersession and consolidation Data Model

Two structures: a frontmatter field and the annotation block it implies. No database — the runtime has none, and §runtime-boundary forbids adding one without a constitutional amendment.

## Frontmatter field

```yaml
supersedes: [005-workflows, 019-config-decisions]
```

| Property | Value |
| --- | --- |
| Required | no — absent means this spec supersedes nothing |
| Type | list of feature slugs |
| Authored by | the **operator**, via a declaring command |
| Empty form | the key is **absent**, never `[]` |

The empty form is deliberately unlike `dependencies:`, which the frontmatter schema requires present-and-empty, and deliberately like `references:`, which is absent when empty. The reason is what the key asserts: `dependencies: []` is a derived index truthfully reporting "harvested, found none", so its presence is evidence the derivation ran. `supersedes:` is hand-authored and derived from nothing, so an empty list would assert a considered decision nobody made.

### Arity

A list from the outset. One spec routinely supersedes several at once — `043-workflows-sunset` names four sibling specs in its removal surface. This is the deliberate divergence from `folds-into:`, which is scalar because a staging spec has exactly one home.

### Validation

| Stage | Checks |
| --- | --- |
| `validate-frontmatter` | **Shape only** — each entry parses as a feature slug |
| Declaring command | **Existence** — each named feature directory holds a `spec.md` |

The split mirrors `folds-into:`, whose shape is checked at creation while existence is enforced where the consequence lands. It diverges on *where*: a fold target normally lives on another branch and cannot be resolved until fold-back, whereas both specs in a supersession exist at declaration time, so that is the first and correct moment to refuse.

A self-referencing entry is rejected — a spec cannot supersede itself, and unlike `derive-dependencies`, which records self-links so its cycle check can surface them, there is no later pass here to catch it.

## Annotation block

Written into the **superseded** spec's body, immediately after the H1 and its lead paragraph, ahead of any annotation already present.

```markdown
> **Sunset ([NNN-slug](../NNN-slug/spec.md)):** {authored substance — what no longer holds}
> This spec stays `done` as the historical record of what shipped; {scope statement}.
```

| Element | Source |
| --- | --- |
| Blockquote prefix | frame — **load-bearing** |
| `**Sunset ([link]):**` citation | frame |
| What no longer holds | **authored** |
| Record-of-what-shipped closer | frame |

### Why the blockquote is structural

`derive-dependencies` does not harvest links on blockquote-prefixed lines. The wrapper is therefore what allows the annotation to link its superseding spec without the superseded spec acquiring a dependency on its own successor. `specs/005-workflows/spec.md:20` demonstrates it: the sunset note links `043-workflows-sunset`, and the frontmatter names only `004` and `010`.

A **criterion-level** annotation gets no such exemption — a plain list item's link *is* harvested — so it cites by name and points at the banner that carries the link, exactly as `specs/005-workflows/spec.md:90` does.

### Accumulation

Annotations stack newest-first and are never replaced. `005-workflows` carries four, and its sunset banner *scopes* the three post-completion notes below it rather than deleting them. Replacing an earlier annotation would destroy the record of an intermediate state, which is the thing the annotation exists to preserve.

## Notes

- The annotation is a **mechanical edit** under §spec-lifecycle: it changes no claim the spec makes about what it delivered, so the superseded spec keeps its status. This is what allows it to be written without reopening a `done` spec.
- Phrase the substance as a **non-claim** ("no longer exists", "is removed") rather than as an assertion whose truth depends on whether its paths resolve — `check-artifacts`' `criterion-path-existence` family classifies such text as `not-a-live-claim`, and a live-claim phrasing would fall under a check the annotation was never meant to answer to.
