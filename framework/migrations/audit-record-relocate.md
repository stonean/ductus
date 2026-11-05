# audit-record-relocate

**Introduced in:** ductus 0.50.0
**Summary:** Move each spec's `review:` and `analyze:` frontmatter blocks into the artifacts that own them — `review.md` and a new `analysis.md` — so each audit record has one home.

## Background

The review record was written twice. `spec.md` carried a `review:` block and the `review.md` beside it carried its own frontmatter, and in spec 047 seven of the nine keys in the block restated what the report already said. Nothing held the two together, and they drifted: specs 031 and 041 carried `should-violations: 1` in the block while their reports recorded `0`, invisibly, for weeks — because every gate reads exactly one of the two files.

The analyze record had the opposite shape. There was no artifact at all, so `spec.md` frontmatter was its only home, carrying a per-path digest map that grew with the size of the spec directory. Measured across this repository's 54 specs, `spec.md` opened with an average of 37.5 lines of frontmatter before its first line of prose.

After this migration `spec.md` carries neither block. [Spec 057](../../specs/057-analyze-artifact-and-record-relocation/spec.md) governs; the constitution's **Frontmatter Schema** declares both records at their new homes.

## Procedure

1. **Idempotency check.** The relocation primitive reports `changed: false` for a spec carrying no block, so a converged project produces no writes. Run the sweep unconditionally; there is no separate probe to perform.

2. **Run the primitive, one spec at a time.** For each spec directory under the configured spec root (`[paths] specs-root`, default `specs`), invoke `relocate-audit-records` against the feature. The primitive:
   - writes each block as top-level frontmatter on its artifact — `review:` to `review.md`, `analyze:` to `analysis.md`;
   - **merges** into an artifact that already exists rather than refusing, which is the normal case for `review.md` since every pre-migration spec has one, and carries that artifact's existing report body **verbatim**;
   - folds the pair `reviewed-at` / `last-run` onto `last-run`. They are one instant spelled twice — the block called it `last-run`, `review.md` called it `reviewed-at` — and carrying both forward would preserve the two-names condition inside the artifact that exists to end it. A disagreement across the pair is named under the spelling the artifact used;
   - **names** every key where the block and the artifact disagreed, in `disagreements`, rather than resolving it quietly. The block wins, because it is the copy every gate actually read — but 031 and 041 are why the difference is surfaced instead of smoothed. Report these to the operator;
   - invents no record for a spec that carries no block, because a record of nulls would assert a run that nothing substantiates;
   - removes the blocks from `spec.md`, leaving every other frontmatter key and the entire body untouched.

   The sweep is performed by the primitive, never by hand. A hand edit across 54 spec directories is a silent corruption waiting to happen, and a shell script doing it would have to parse frontmatter, which [§runtime-boundary](../constitution.md#runtime-boundary) principle 3 rules out.

3. **Leave `done` specs at `done`.** The edit is confined to the frontmatter block: no criterion text changes, no body prose changes. That is a correction under [§spec-lifecycle](../constitution.md#spec-lifecycle), so the back-edge does not fire and a `done` spec stays `done`. The primitive enforces this rather than trusting it — it compares the body before and after and refuses to write if they differ. A run that finds itself changing prose is not this migration and must stop.

4. **Skip a pinned spec.** A spec listed in `.ductus/config.toml` `[pinned] files` is left untouched, with one line naming it — pinning opts a file out of framework writes, and this is a framework write:

   `warning: {file} is pinned; leaving its record blocks in spec.md — run relocate-audit-records by hand if you want them moved.`

   A pinned spec keeps a residual block, which `validate-frontmatter` reports at the Blocking tier. That is the correct outcome: the finding is how the operator learns their pin has a consequence.

5. **Check the CI file, which this migration cannot fix.** `/{project}` does not install CI files — that would require platform detection beyond its scope — so an adopter holds a *copy* of the shipped template. The pre-relocation copy skips any spec whose frontmatter has no `review:` block, on the reasoning that such a spec predates the review feature. **After this migration that predicate is true of every spec**, so the step does not go stale: it goes green across the whole corpus while checking nothing.

   Search the project for a CI file matching the old shape — a workflow containing both `status: done` and a `grep`-style test for `^review:` — and if one is found, print:

   ```text
   warning: {file} carries the pre-relocation review-blocking gate, which now
   passes for every spec because no spec has a `review:` block. Re-copy
   framework/templates/ci/adopter-generators.yml.
   ```

   This is a notice, not a fix. The migration must not rewrite a file the adopter owns and `/{project}` never installed.

## What this migration leaves stale, and why it is not repaired

**Every migrated spec's recorded `analyzed-digest` stops matching, and that is
expected rather than a defect.** The digest is a per-path hash of the artifacts
the analysis read; this migration rewrites `review.md` — a subject of that
digest — in every spec that had a review record. So the recorded digest and the
tree disagree from the moment the sweep lands.

Measured on this repository after its own sweep: **50 of 54** `done` specs
mismatch on `review.md`, which is the migration's doing.

**Read that alongside a larger effect the migration does not cause.** All 54
also mismatch on `spec.md`, and none of that is this migration: a spec's
completing `set-status` rewrites `spec.md` *after* its analysis was recorded, so
every `done` spec's digest has been stale on that one path since long before
this change. Verified by re-hashing a spec's `spec.md` with `status:
in-progress` restored, which reproduces the recorded digest exactly. It is the
state `/{project}:implement`'s pre-done gate handles by returning early on a
spec that is already `done`.

**Nothing is blocked and nothing needs doing.** `check-review-gate` short-
circuits on a `done` spec, and no `/{project}:audit` family reads analyze
freshness, so the stale digests are latent. They clear on the next
`/{project}:analyze` of any spec that is reopened, which is the normal cost of
reopening a spec rather than an extra one.

**Do not backfill the digests.** Rewriting them would assert that an analysis
read the post-migration tree when none did — the reasoning 047's AC11 already
used to reject backfilling the record itself, applied to one of its fields.

## Verification

- No `spec.md` under the spec root contains a top-level `review:` or `analyze:` key.
- Every spec that had a record has the corresponding `review.md` or `analysis.md`, and each one's frontmatter deserializes into the record.
- Specs that had no record have no new artifact.
- `git diff` over the sweep shows frontmatter changes only — no body line moves.
- Any `disagreements` and any CI-file warning were reported to the operator, not swallowed.
