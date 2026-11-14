---
section: "Derived frontmatter indexes"
---

# Derive-references-results-are-bounded

## Context

`derive-references` returns its spec lists — `updated`, `unwritten`, `untracked-skipped`, `absent`, `unparseable` and `unwritable` — inline whatever their length, and each grows with the corpus: a service rename drifts every referencing spec at once, and under `staged` each lands in `unwritten` ([derive-references-unstaged-drift-is-reported](derive-references-unstaged-drift-is-reported.md)). `derive-dependencies`, whose result holds the same kinds of list, moves them to a file past `[artifacts] read-size-bytes` (`specs/080-deterministic-runtime-efficiency/data-model.md` §Bounded results). `derive-references` calls no `bounded_lists::place` and its result carries no `form` (`runtime/src/primitives/derive_references.rs`; `DeriveReferencesResult` in `runtime/src/schema/primitives.rs`). 080's measurement found it did not grow because its corpus was in sync and held no references, and it measured `derive-dependencies`' drifted case but not this one (`specs/080-deterministic-runtime-efficiency/research.md` §Result sizes).

Found by 017's analysis on 2026-11-14 as `BE-PAYLOAD-001` (MUST) on 017's scenario `generator-sync-claim-honesty`, which describes both derivations' lists; the owner placed the bound here the same day.

## Behavior

- `derive-references` holds its result to `[artifacts] read-size-bytes` in the shape `derive-dependencies` takes (`specs/080-deterministic-runtime-efficiency/data-model.md` §Bounded results): inline while it fits, and past it each of the six lists moves to the file `lists-file` names, with its count inline in both forms and `unwritable`'s exception as `derive-dependencies` has it.
- The CLI gates and reports on the whole derivation, as `derive-dependencies`' does: a spec in `unwritable` moved to the file still makes the CLI exit non-zero and is still named on stderr.
- A writing run has written by the time its result is bounded, so a list file that cannot be written fails a call whose writes stand.
- The primitive's compiled description and this spec's data model, §`derive-dependencies` / `derive-references`, state the bound.

## Edge Cases

- A corpus with no references, or no `[services]` registry, returns every list empty and the result inline, as today.
- A result exactly at the read size fits, and is inline.
- The pre-commit hooks, this repository's and the shipped one, send the result to `/dev/null` and read only the exit status and stderr, so the bound changes nothing they see.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
