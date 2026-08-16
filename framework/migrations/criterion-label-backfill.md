# criterion-label-backfill

**Introduced in:** ductus 0.28.0
**Summary:** Assign a stable `AC{n}:` label to every unlabelled acceptance criterion across the project's specs, and record the `next-criterion` counter that keeps a retired label from being reissued.

## Background

Spec 013 makes an acceptance criterion citable by a permanent identifier rather than by its position in a list: `AC7` still names the same requirement after six criteria are inserted above it, and a reader citing one never has to count. `label-criteria` assigns the labels and maintains a `next-criterion` counter in the spec frontmatter.

The framework's own corpus was swept when the primitive landed. Without this entry, this repository is labelled and every adopter is not — the ecosystem-level version of the two-tier corpus that the going-forward-only option was rejected for. An adopter would read documentation citing `AC12` while their own specs number nothing.

`introduced_in` is `0.28.0` because that is the release that first carries `label-criteria`. An adopter reaching this migration on an older runtime would have no primitive to run, which is why the entry could not be authored ahead of the release. From `0.28.0` on the question cannot arise: `/ductus` acquires the pinned runtime in its **Pre-flight Phase**, which runs before **Pre-run Migrations**, so the binary is present by the time this executes.

## Procedure

1. **Idempotency check.** Run the labelling pass in its reporting form across the project's spec root. If every criterion in every spec already carries a label and every counter is current, exit silently — the pass is idempotent by construction, so a converged project produces no writes.

2. **Run the pass, one spec at a time.** For each spec directory under the configured spec root (`[paths] specs-root`, default `specs`), invoke `label-criteria` against the feature. The primitive:
   - assigns `AC{n}:` between the checkbox and the text of each unlabelled criterion, in body order;
   - takes each new label from `max(highest label in the body, next-criterion)`, never `max(body) + 1`, so a label retired by a deleted criterion is never handed to a different requirement;
   - writes the advanced `next-criterion` into the frontmatter;
   - leaves an already-labelled criterion **byte-identical**.

   The sweep is performed by the primitive, never by hand — 700 criteria across 49 specs in this repository's own backfill, where a hand edit is a silent renumbering waiting to happen.

3. **Leave `done` specs at `done`.** The diff assigns labels and maintains the counter; each labelled criterion's own text is byte-identical either side of it. That is case (c) of the mechanical-edit rule in [§spec-lifecycle](../constitution.md#spec-lifecycle), so the back-edge does not fire and a `done` spec stays `done`. A run that finds itself changing criterion *text* is not this migration and must stop.

4. **Skip a pinned spec.** A spec listed in `.ductus/config.toml` `[pinned] files` is left untouched, with one line naming it — pinning opts a file out of framework writes, and this is a framework write:

   `warning: {file} is pinned; leaving its criteria unlabelled — run label-criteria by hand if you want them numbered.`

5. **Summary line.** When anything was labelled, report `labelled acceptance criteria: {N} criteria across {M} spec(s)` in the post-scaffolding output. Omit the line entirely when nothing changed.

## Notes

- The migration is one-way. There is no reverse path — and none is wanted: stripping labels would renumber from the advanced counter and open a gap for no gain.
- **Unlabelled criteria keep working.** `mark-criterion` addresses a criterion by 0-based index as well as by label, so an adopter who skips this migration (or pins a spec) loses citability, not function. That is what makes this a convergence rather than a breaking change.
- Two specs in this repository's backfill were labelled by hand beforehand and came out with their criterion bytes untouched; their only diff was the `next-criterion` line the counter requires. An adopter who has hand-labelled a spec gets the same treatment.
- The pre-commit hook installed by `/ductus` runs the same pass as a backstop for a criterion typed by hand in an editor, so a project converged here stays converged without anyone remembering to re-run anything.
