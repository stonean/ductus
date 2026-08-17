---
section: "Follow-on scenarios"
---

# Orphaned-reference-check

## Context

[027](../../027-bootstrap-migration-registry/spec.md)'s
`migration-chain-reference-integrity` requires that an adopter-owned file left
pointing at a framework-owned path which no longer exists is **reported**. Two
real instances surfaced during the adopter bootstrap 048 AC10 called for: a
`CLAUDE.md` `@import` left dangling by `ductus-rename` moving the file
`constitution-relocate` had pointed it at, and a `.githooks/pre-commit` calling
generators two migrations had relocated out from under it.

That scenario settles *what* the rule is and *where* it is surfaced — the
durable home is `/{project}:analyze` §Project-level consistency, with the
bootstrap's migration batch as a second call site. This scenario is the runtime
half: the primitive both call sites invoke, so one rule has one implementation
rather than two that can disagree (the Family 19 / `mechanical_sweep` pairing is
what that costs when it is unavoidable; here it is avoidable).

## Behavior

**`check-orphaned-references` reports adopter-owned files whose references to
framework-owned paths do not resolve.** It reads the adopter-owned referrer set,
extracts the framework-owned paths each names, tests each against the
filesystem, and returns one finding per path that fails to resolve — the
referring file, the missing path, and the line. Read-only: it reports, never
repairs, because the adopter may have hand-edited the reference and a wrong
rewrite is worse than a precise report.

**Attribution has two modes and the result names which one ran.** When
`framework/migrations.toml` is readable — the bootstrap call site, where the
registry sits in staging — each finding carries the entry whose `target_paths`
covers the missing path, and the result reports `attribution: "registry"`. When
it is not — the `analyze` call site in an adopter checkout, where the registry
is not installed — findings carry no entry, the result reports
`attribution: "watermark"` and echoes `.ductus/config.toml`'s
`[migrations].last_applied` as the only migration context available. The two
render differently. Emitting a watermark-only finding in the shape of an
attributed one would be `QUAL-CLAIM-001` inside the check written to enforce
that principle.

**What it could not examine is reported, not dropped.** A referrer the primitive
could not read lands in `skipped` with its reason, never in a clean finding
count — an empty `findings` array means *examined and found nothing* only when
`skipped` is empty, the same contract `check-artifacts` and
`derive-routing-candidates` already carry.

## Edge Cases

- **A converged adopter**: nothing is orphaned, so `findings` is empty and
  `skipped` is empty — a real clean result, distinguishable from an unexamined
  one.
- **A reference the adopter hand-edited to something valid**: resolves, so it is
  not reported. The check tests reachability, not conformance to a shipped form.
- **A path that is legitimately absent** — an optional artifact never
  scaffolded: reported, because the check cannot tell it from a break. A
  suppression list would be the declared input 027's scenario rules out.
- **A pinned referrer**: still reported. Pinning opts out of framework *writes*,
  not out of being told the file is broken.
- **The registry is present but unparseable**: `attribution: "watermark"` with
  the parse failure in `skipped` — not silently downgraded, since a caller
  expecting attribution at the bootstrap call site must be able to tell a
  missing registry from a broken one.
- **No `[migrations]` section at all** (an adopter who has never run a
  migration): `last_applied` is null and the result says so; it is not rendered
  as an empty string that reads like a migration named "".

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
