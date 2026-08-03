---
section: "Follow-on scenarios"
---

# Criterion-path-existence-family

## Context

An acceptance criterion is a contract: naming a path in one asserts that path is part of the delivered system. Nothing re-verifies that assertion after the spec reaches `done`, so a later spec can delete the subject and leave the criterion checked.

That is not hypothetical. Spec 026 reached `done` with a criterion naming `framework/workflows/registry.json` and `scripts/audit/registry-equivalence.sh`; spec 043 later deleted both, unchecking nothing. The completion gate would have passed 026 at full marks had the criteria not been read against the filesystem by hand. It is a sharper failure than stale prose — stale prose misleads a reader who can still discount it, while a stale contract is *treated as satisfied* by the tooling.

[045 — Decision-state drift detection](../../045-decision-state-drift-detection/spec.md) owns the requirement; this scenario carries the runtime work, per that spec's Implementation ownership split. It shares none of [link-adjacent-drift-family](link-adjacent-drift-family.md)'s machinery — see the parsing inversion below — and depends only on [check-artifacts-skipped-targets](check-artifacts-skipped-targets.md).

## Behavior

**A seventh `check-artifacts` family, `criterion-path-existence`, advisory.** For a spec at `status: done`, each entry under `## Acceptance Criteria` is scanned for filesystem paths, and each path that no longer resolves produces one finding naming the criterion and the path. Criteria come from `read-spec`'s parsed acceptance-criteria list rather than a second section walker.

**It reads inside inline code spans — the inverse of the family above.** Paths are backticked by convention, which is exactly the context the tell scan must ignore. One family cannot hold both rules coherently, which is why these are two families rather than one check with a flag.

**A code span's whole trimmed content is a candidate path** — with surrounding quotes and a leading `./` stripped — when it contains a `/` that is not its final character, contains no whitespace, contains none of `{ } * ? [ ] < > $ |` or `:`, does not begin with `-` or `/`, does not contain `NNN`, and is entirely ASCII. Each exclusion earns its place against real acceptance-criteria text: `:` rejects URLs, `path:line` citations, and every slash-command reference; the braces reject placeholders and `NNN` the unbraced spec-number one; the bracket and star forms reject globs; a leading `-` rejects flags; the ASCII test rejects the `…` of an elided path; and requiring an *internal* separator rejects a bare directory name used conceptually ("the feature's `scenarios/` directory"). A candidate resolves when the repo-root-relative path exists as a file or a directory, with a trailing `/` stripped first.

**A path is checked only when its criterion claims the path is present.** Thirteen closed phrases mark a criterion as something else — deletion and retirement, rename, adopter scope, and hedges — and exempt it whole, recording each of its paths as `not-a-live-claim`. This is the open-state tell list's co-occurrence design inverted: there a phrase asserting an open state is contradicted by a closed target; here a phrase asserting absence is *confirmed* by a path that does not resolve, so a finding would be exactly backwards. `framework/commands/capture.md is deleted` is satisfied precisely because that file is gone. The phrases are matched as phrases rather than words, because bare `adopter` would exempt a criterion naming a genuinely stale path.

**An unresolved candidate is a finding only when its own top-level segment exists here.** When that segment is absent, nothing is provable — a framework repo's criteria legitimately name paths living in an *adopter's* checkout — so the candidate is recorded as `root-absent` rather than flagged. Recorded, not exempted: the report says what went unexamined instead of quietly reading as clean. The rule self-corrects where it matters, since in an adopter repo those roots exist and drift beneath them is provable again.

**Scope is `## Acceptance Criteria` on `done` specs, and nothing else.** Body prose may name a dead path perfectly correctly while describing history — 026's own Behavior section now reads that spec 043 "deleted `framework/workflows/`", a true statement about a path that is supposed to be gone. Widening this check to whole spec bodies would flag true statements, which is why the contract framing is load-bearing rather than incidental.

## Edge Cases

- A criterion naming a path that resolves to a **directory** is satisfied; `framework/workflows/` is a directory reference, not a malformed file path.
- A criterion naming no paths at all — the common case — contributes nothing.
- A spec below `done` is not scanned. Its criteria describe work in flight, so a path that does not exist yet is expected rather than drifted.
- An unbackticked path in a criterion is missed. Accepted: outside code font a `/`-bearing token is more often a slash command, a placeholder, or an `and/or` than a path, and the originating case and every path in this repo's criteria are backticked.
- An absolute path is rejected by the grammar rather than resolved against the filesystem — the check makes claims about this repository only.
- The family reads no targets on a spec below `done`, so it records nothing in `skipped`: not applicable is distinct from tried-and-failed.
- **Residual, measured rather than assumed:** an adopter-layout path whose top-level segment *also* exists in the framework repo (`.govern/constitution.md`, `specs/rules/security-backend.md`, `.githooks/govern-pre-commit`, `specs/system.md`) still flags, because `root-absent` cannot see that the criterion describes a different checkout. Nineteen of the 28 findings in the 2026-08-03 full-repo run are this class — and every one of those paths is something `govern` *creates in an adopter project*, absent here only because `govern` is the source rather than an adopter. The class is structural to a framework repo documenting adopter layout while also being its own adopter, and it does not generalize to the projects this check ships to. It is why the promotion verdict reads *do not promote, and re-measure in an adopter repo*: this repo is the least representative sample the check will ever run against.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
