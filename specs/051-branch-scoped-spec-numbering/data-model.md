# 051 — Branch-scoped spec numbering Data Model

The feature introduces one directory-name grammar, one parsed form, one frontmatter field, and argument/result additions to `create-feature` plus three new primitives. No database is involved — the durable structures are on disk.

## Directory-name grammar

```text
feature-dir  := sequential | branch-scoped
sequential   := NNN "-" any                   ; NNN = three ASCII digits
branch-scoped:= identifier "." n "-" slug
identifier   := segment ("-" segment)*        ; excludes "." by construction
slug         := segment ("-" segment)*
segment      := [a-z0-9]+
n            := [1-9][0-9]*                   ; no leading zeros
any          := one or more characters        ; not held to the slug grammar
```

The parse splits `branch-scoped` on the **first** `.`. That is unambiguous because `identifier` cannot contain one: the operator's input is sanitized to `segment ("-" segment)*` before it is used, collapsing any `.` to a hyphen.

`sequential` is unchanged from today and is recognized by the absence of a `.`.

**The two forms are held to different standards, deliberately.** The branch-scoped form is machine-generated end to end — `create-feature` sanitizes the identifier and derives the slug — so both halves are validated against the slug grammar. The sequential form's trailing slug is accepted as-is, exactly as it is today: that form predates the grammar's enforcement, and an adopter's spec root may hold a directory that would fail it. Tightening the rule there would make such a directory invisible to every corpus reader at once — a silent regression rather than a reported one — so the legacy form keeps the legacy leniency.

## Parsed form

```rust
enum FeatureForm {
    Sequential { number: u32 },
    BranchScoped { identifier: String, n: u32 },
}

fn parse_feature_dir(name: &str) -> Option<FeatureForm>;
```

Replaces the `is_feature_slug` / `feature_number` pair. `is_feature_slug(name)` becomes `parse_feature_dir(name).is_some()`. A caller wanting a sequential number matches on `Sequential`, so a branch-scoped directory can no longer yield one — which is what removes today's `1234.1-slug` → `123` misparse.

## Frontmatter

One optional declared key on a branch-scoped spec:

```yaml
---
status: draft
dependencies: []
folds-into: 022-deterministic-runtime
---
```

| Key | Type | Presence | Written by | Read by |
| --- | --- | --- | --- | --- |
| `folds-into` | string — a sequential feature directory name, which may not exist in this branch | Optional, but chosen explicitly; absent means "no upstream home" (the renumbering case) | `create-feature` at creation; `rewrite-spec-links` on a rename | fold-back, `check-unfolded-specs`, `validate-frontmatter` |

**Declared, not derived.** Unlike `dependencies:` and `references:`, nothing in the repository can derive this value — it records intent stated nowhere else. It survives every existing frontmatter writer because each splices only its own key rather than re-serializing the block.

**Validation — shape only.** `validate-frontmatter` reports a finding when `folds-into` is present and does not *parse* as a sequential feature name (`NNN-slug`), which forbids chaining one branch-scoped spec into another. It never checks that the named feature exists: the target normally lives on the upstream branch and is absent from the tree declaring it, so an unresolvable target is the expected state before a merge, not a defect. Existence is enforced at fold-back by `retire-feature`.

An absent key is never a finding either: a sequential spec has no fold target, and a branch-scoped spec may legitimately declare none.

## Primitive argument and result shapes

### `create-feature` (extended)

| Field | Direction | Type | Notes |
| --- | --- | --- | --- |
| `branch-id` | in | optional string | Absent → sequential numbering, unchanged. Present → branch-scoped. |
| `fold-into` | in | optional string | Written to `folds-into:`. Requires `branch-id`. |
| `identifier` | out | optional string | The **sanitized** identifier actually used, so the caller can echo it before the directory exists. |
| `feature`, `path`, `created`, `template` | out | unchanged | `created: false` remains the already-exists domain outcome. |

### `rewrite-spec-links` (new)

| Field | Direction | Type | Notes |
| --- | --- | --- | --- |
| `from` | in | string | The retiring or renamed feature directory name. |
| `to` | in | string | The fold target — a feature, optionally with a scenario slug. |
| `rewritten` | out | list of `{path, count}` | Files whose body links or `folds-into` fields were re-pointed. |
| `examined` | out | integer | Files scanned, so an empty `rewritten` is not read as "nothing was checked". |

### `retire-feature` (new)

| Field | Direction | Type | Notes |
| --- | --- | --- | --- |
| `feature` | in | string | Must parse as `BranchScoped`; a sequential feature is refused. |
| `fold-target` | in | string | Must exist. Refusal when it does not is what prevents stranding content. |
| `retired` | out | boolean | `false` is the domain outcome for "already gone", not an error. |

### `check-unfolded-specs` (new)

| Field | Direction | Type | Notes |
| --- | --- | --- | --- |
| `unfolded` | out | list of `{feature, identifier, folds-into, status}` | `folds-into` null when the spec declares none. |
| `examined` | out | integer | Directories scanned. |

## Notes

- Every count field above exists so an empty result reads as "examined and found nothing" rather than "nothing was examined" — the same distinction `derive-routing-candidates` draws with `sources-examined` / `skipped`.
- No structure here is shared with another spec's data model, so there is no cross-spec structural conflict to reconcile.
