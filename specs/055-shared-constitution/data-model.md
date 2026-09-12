# 055 — Shared Constitution Data Model

Canonical schema for the `[constitutions.<alias>]` registry. Mirrors the split
`030-cross-service-references` uses, where `data-model.md` is the canonical source for
the `[services]` schema and the command source points at it rather than restating it.

## Config schema

```toml
[constitutions.acme]
repo = "https://github.com/acme/governance"
path = "../governance"
description = "Acme engineering house rules"
```

| Field | Required | Constraints |
| --- | --- | --- |
| `<alias>` | yes | A bare TOML key — letters, digits, hyphens, underscores; no whitespace, dots, or quotes. Unique within `[constitutions]`. Duplicate aliases are a TOML error, not a ductus one. |
| `repo` | yes | URL-shaped (a scheme and a host). Recorded verbatim. **Identity and navigation only — never fetched.** |
| `path` | yes | Local checkout location, relative to the repo root or absolute. `..` is permitted; a sibling checkout is the normal case. Recorded exactly as written. |
| `description` | no | Free text: what the source governs. No *resolution* behavior depends on it — it never changes which documents load — but it is carried through to every surface that names a source (`/{project}:target`'s loaded and skipped reports, `write-review`'s `## Unexamined governance` section), because an alias is a config key someone chose and does not answer the question attributability asks. Absent is absent: an entry without one renders as it did before. Internal whitespace is collapsed by `resolve-constitutions` so a multi-line value cannot break a single-line report. |

The document read from a resolved checkout is `{path}/constitution.md`. Nothing else in
the checkout is read — not its `rules/`, not its own `.ductus/config.toml` (see the
spec's Edge Cases: registration is not transitive).

## Resolution outcomes

Each entry resolves to exactly one outcome. The two failure outcomes are distinct
because the spec requires distinct messages for them.

| Outcome | Condition | Reported as |
| --- | --- | --- |
| `loaded` | `path` resolves to a directory containing `constitution.md` | `loaded[]` with alias, checkout path, document path |
| `not-checked-out` | `path` does not resolve to a directory | `skipped[]` with alias and reason |
| `no-constitution-document` | `path` resolves, but holds no `constitution.md` | `skipped[]` with alias and reason |

An absent `[constitutions]` table and a present-but-empty one both yield empty `loaded`
and empty `skipped` — indistinguishable by design (AC1).

## Primitive result shape

`resolve-constitutions` returns:

| Field | Type | Meaning |
| --- | --- | --- |
| `loaded` | array | Entries that resolved, in alias order |
| `skipped` | array | Entries that did not, each with its `reason` |
| `examined` | integer | Total entries considered — the denominator for `loaded` |
| `duplicate-paths` | array | Aliases sharing a checkout `path`, if any — `{path, aliases}` groups |

The `loaded` / `skipped` split is the `QUAL-CLAIM-001` shape: a caller cannot report a
clean result over a source in `skipped` without dropping a field it was handed. Empty
`loaded` **and** empty `skipped` means "no constitutions registered"; empty `loaded`
with a non-empty `skipped` means "registered and could not be read" — different answers,
never collapsed.

## Notes

- Ordering is **alias order**, from the registry's `BTreeMap`. AC7 requires two projects
  with the same entries over the same checkout content to load the same documents in the
  same order on any machine. Alias order delivers that; config order would not, because
  it does not survive a TOML reformat — and hash-map traversal order would not either.
- Two entries naming the same `path` are warned and allowed; the document loads once.
  This matches `/ductus:link`'s duplicate-`repo` posture (warn, do not block).
- No `ref`, `version`, or revision field exists. The checkout's own git state is the
  version, and ductus never fetches — see the spec's Resolved Questions.
