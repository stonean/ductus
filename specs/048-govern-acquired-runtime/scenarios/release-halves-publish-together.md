---
section: "Acquisition"
---

# Release-halves-publish-together

## Context

A release has two halves — the GitHub release carrying the platform binaries,
and the crates.io publish carrying the source crate. Nothing makes them
atomic, so a failure in the second leaves a version that is half-released,
and the recovery is always a manual re-run someone has to notice.

`runtime-release.yml` gates `publish` correctly: `needs: [audit, build,
release-assets, acquire, sbom]`. What it does **not** do is gate the GitHub
release on `publish`. So the ordering is release-then-publish, and a publish
failure cannot roll the release back.

This has now happened twice, both recorded in the workflow's own comments or
history:

- **`ductus-v0.29.9`** — the SBOM upload hit a transient GitHub 5xx, the job
  failed, and the crates.io publish went ahead regardless, because `sbom` was
  a *sibling* of `publish` rather than a dependency. The release sat with ten
  binaries and no SBOM until the failed job was re-run by hand. Fixed by
  adding `sbom` to `needs` — which fixed that instance, not the class.
- **`ductus-v0.30.0` and `ductus-v0.31.0` (2026-08-19)** — an expired
  `CARGO_REGISTRY_TOKEN` failed `cargo publish` with `403 Forbidden` on both.
  Both GitHub releases published and were downloadable; neither crate was.
  `0.30.0` was recovered by `gh run rerun --failed` after the token was
  rotated; `0.31.0` needed a second re-run because its publish job had already
  started and captured the stale secret at job start.

**Both 2026-08-19 releases are fully recovered — nothing operational is
outstanding.** crates.io reports `max_version: 0.31.0`, and the GitHub
releases for `ductus-v0.30.0` and `ductus-v0.31.0` are live with all five
platform assets and their SBOMs. This scenario is about the *class*, not about
repairing those two; a session picking it up should change the workflow, not
re-run anything.

The shape is the same each time: the irreversible half succeeds, the
recoverable half fails, and the release is left inconsistent in the direction
that is hardest to notice — because the GitHub release is the half adopters
see, so nothing downstream complains.

Worth stating plainly, because it bounds the fix: **crates.io is the
irreversible half.** A published crate version cannot be unpublished, only
yanked. A GitHub release can be deleted and recreated. So any ordering that
makes the two consistent has to attempt the irreversible one first.

## Behavior

`publish` runs **before** the GitHub release is created, and the release job
depends on it. A failed `cargo publish` therefore leaves no GitHub release at
all, rather than a downloadable release whose crate is missing.

The `needs` chain becomes `audit → build → acquire → sbom → publish →
release-assets`, so every gate that guards the current publish still guards it,
and the release becomes the last thing that happens rather than the first
irreversible one.

Two consequences the scenario accepts deliberately:

- **A failed release leaves nothing user-visible.** That is the point. The
  operator re-runs the tag pipeline rather than reconciling two halves by hand,
  and no adopter ever sees a version that exists in one place and not the other.
- **A `cargo publish` that succeeds and a release-assets step that then fails**
  leaves a crate with no binaries — the inverse split. It is strictly better
  than today's: `release-assets` is retryable and its inputs are already built
  artifacts, whereas a missing crate publish for an already-tagged version is
  the case that needed two manual re-runs this session.

The acquisition invariant is unaffected: `acquire` still runs against the
built assets before either publish, so a release that cannot be acquired still
fails before anything is published.

## Edge Cases

- **A secret captured at job start.** GitHub Actions resolves secrets when a
  job begins, not when a step runs, so rotating a token mid-run does not reach
  a job already in flight. This is why `0.31.0` needed a second re-run after
  `0.30.0` had already proven the new token worked. Not fixable in the
  workflow; worth naming so the next operator does not read it as the token
  still being wrong.
- **Re-running `publish` for a version already on crates.io** fails with
  "crate version already uploaded". That is a safe, idempotent-enough outcome
  — it is not a partial write — but the job's failure is then indistinguishable
  from a real one unless the operator reads the log.
- **The workflow's existing comment about `0.29.9` stays.** It records why
  `sbom` is in `needs`, which remains true and is a different reason from this
  reordering.
- **Ordering does not make the halves transactional.** Nothing can, across two
  registries. The claim is narrower and is the one worth holding: the
  irreversible half is attempted first, so the recoverable half is the only one
  that can be left undone.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
