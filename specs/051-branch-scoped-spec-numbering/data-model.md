# 051 — Branch-scoped spec numbering Data Model

The feature introduces one directory-name grammar, one parsed form, one frontmatter field, four new primitives, and argument/result additions to `create-feature` and `append-task`. No database is involved — the durable structures are on disk.

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
| `folds-into` | string — a sequential feature directory name, which may not exist in this branch | **Required** on a branch-scoped spec, absent on a sequential one | `create-feature` at creation; `rewrite-spec-links` on a rename | fold-back, `check-unfolded-specs`, `validate-frontmatter`, `dashboard` |

**Declared, not derived.** Unlike `dependencies:` and `references:`, nothing in the repository can derive this value — it records intent stated nowhere else. It survives every existing frontmatter writer because each splices only its own key rather than re-serializing the block.

**Validation — shape only.** `validate-frontmatter` reports a finding when `folds-into` is present and does not *parse* as a sequential feature name (`NNN-slug`), which forbids chaining one branch-scoped spec into another. It never checks that the named feature exists: the target normally lives on the upstream branch and is absent from the tree declaring it, so an unresolvable target is the expected state before a merge, not a defect. Existence is enforced at fold-back by `retire-feature`.

An absent key is never a finding on a *sequential* spec, which has no fold target by definition. A branch-scoped spec always carries one — creation refuses without it — so an absent key there means the spec was hand-edited, which is the supported way to make it stand on its own.

## Primitive argument and result shapes

### `create-feature` (extended)

| Field | Direction | Type | Notes |
| --- | --- | --- | --- |
| `branch-id` | in | optional string | Absent → sequential numbering, unchanged. Present → branch-scoped. |
| `fold-into` | in | string | Written to `folds-into:`. **Required** with `branch-id`, refused without it. |
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

> **Post-completion note (052 — Spec consolidation):** the `feature` row above is no longer the whole rule. `retire-feature` gained an `allow-sequential` argument, off by default, so the sequential refusal is **gated rather than absolute**: `/{project}:consolidate` passes it after naming both specs and confirming the content loss, and nothing else does. `/{project}:fold` — this spec's only caller — does not pass it and has no argument that would, so the refusal behaves here exactly as the row describes. The `fold-target` row is unchanged and applies to both callers.

### `invalidate-review` (new)

| Field | Direction | Type | Notes |
| --- | --- | --- | --- |
| `feature` | in | string | The **upstream** spec whose recorded review no longer describes it — not the branch-scoped one being retired. |
| `invalidated` | out | boolean | `false` is the domain outcome for a spec recording no current review: already in this state, so a re-run converges. |
| `path` | out | string | Repo-relative spec path, present either way. |
| `previous-last-run` | out | optional string | The `last-run` that was cleared, so the caller can say *what* it invalidated. |

Resets `review.last-run` and `reviewed-against` to null, zeroes the counts, and clears `blocking` — the un-reviewed state the pre-`done` gate blocks on. **Waivers survive verbatim**, adopter-authored extra fields included: an invalidation says the review is out of date, not that an operator's recorded judgement about a finding was withdrawn.

It exists because the gate's staleness check diffs the spec's *durable contracts* (`scenarios/*.md`, `data-model.md`) and `spec.md` is deliberately outside that set. A fold routed into the upstream spec's **body** therefore moves no durable contract, and the spec would return to `done` on a review that never saw the code the fold brought with it. The fold knows what the diff cannot see, so it says so rather than leaving the gate to infer it.

### `append-task` (extended)

| Field | Direction | Type | Notes |
| --- | --- | --- | --- |
| `appended` | out | boolean | `false` is the dedup domain outcome: a `slug` was supplied and an existing task already points at `scenarios/{slug}.md`, so `task-number` names that task and `tasks.md` is unchanged. |

Reported separately from `created` because the two answer different questions — `created` is about the *file*, `appended` about the *task*. The dedup keys on the whole rendered pointer, so a slug that is a prefix of an existing one is still its own task; without a slug there is no pointer to key on and the call appends as before. This is what lets an interrupted fold be completed by re-running it rather than doubling the upstream spec's task list.

### `check-unfolded-specs` (new)

| Field | Direction | Type | Notes |
| --- | --- | --- | --- |
| `unfolded` | out | list of `{feature, identifier, folds-into, status}` | `folds-into` null when the spec declares none. |
| `examined` | out | integer | Directories scanned. |

## Notes

- Every count field above exists so an empty result reads as "examined and found nothing" rather than "nothing was examined" — the same distinction `derive-routing-candidates` draws with `sources-examined` / `skipped`.
- No structure here is shared with another spec's data model, so there is no cross-spec structural conflict to reconcile.
