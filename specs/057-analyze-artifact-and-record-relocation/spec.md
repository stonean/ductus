---
status: done
dependencies: [020-code-review, 027-bootstrap-migration-registry, 047-analyze-findings-durability]
next-criterion: 27
cross-spec-impact:
  - 047-analyze-findings-durability
  - 020-code-review
  - 026-framework-self-audit
  - 022-deterministic-runtime
---

# 057 — Analyze artifact and record relocation

`/{project}:analyze` gains a per-spec `analysis.md`, mirroring the `review.md`
that `/{project}:review` already writes, and each command's run record moves out
of `spec.md` frontmatter into the artifact that owns it — one home per record
instead of two.

> **Changed by [058 — Findings route at discovery](../058-findings-route-at-discovery/spec.md).**
> Findings no longer route through the inbox. Each one is fixed, routed, or
> discarded in the run that finds it, and both records dropped
> `captured-issues` for a `dispositions:` map and a `decisions:` list.
> `analysis.md`'s skeleton replaced its captured-issues section with
> `## Fixed in this run`, and each tier section now lists its findings. It is
> still not a triage surface: AC13 holds, and the deciding happens in the run.
> AC12, AC14, the Motivation's routing claim, and the first Resolved Question
> are annotated in place.

## Motivation

`spec.md` carried the state of both audit commands in its own frontmatter. As
[020 — `/ductus:review`](../020-code-review/spec.md) and
[047 — analyze findings durability](../047-analyze-findings-durability/spec.md)
gained capability, those two blocks grew: per-subject digest maps,
examined/scope counts, an `unexamined-by-reason` map, a derived `blocking` flag.
Measured across the 54 spec directories in this repo at the time of writing,
`spec.md` opened with an average of **37.5 lines of frontmatter** before its
first line of prose, and `022-deterministic-runtime` opened with **231**. The
frontmatter grew with the size of the spec directory, because a digest map
carries one entry per subject file.

Most of the review half was already duplicated. Every one of those 54 specs
carried a `review.md`, and `review.md` had its own frontmatter recording the
same run. In 047, seven of the nine keys in `spec.md`'s `review:` block restated
what `review.md` already said — six byte-identical under identical names
(`reviewed-against`, the three violation counts, `examined`, `scope`) and a
seventh, the run instant, under two names (`last-run` in `spec.md`,
`reviewed-at` in `review.md`). Two homes for one fact is the condition
[§drift-prevention](../../framework/constitution.md#drift-prevention) exists to
prevent.

**They drifted, and the framework grew a primitive to cope.**
`runtime/src/primitives/check_review_agreement.rs`, which this spec deleted,
existed for no other purpose than holding the two copies against each other,
and its header recorded what it was built after: specs 031 and 041 carried `should-violations: 1` in `spec.md`
while their reports recorded `0`, and the disagreement stood for weeks. It was
invisible because **every gate reads exactly one of the two files** —
`check-review-gate` and `/{project}:analyze`'s drift check read the block, while
the audit family that resolves `reviewed-against` never compares the counts on
either side of it. A stale non-zero count sent a maintainer back to re-derive
two clean specs before a release tag; a stale zero would have hidden real
findings from the `in-progress → done` transition that trusts them. The
duplication was not a tidiness complaint before this spec — it had already cost
signal, and a primitive plus an audit family were the price of living with it.

The analyze half had the opposite shape. There was no artifact at all, so
`spec.md` frontmatter was the record's only home — not duplicated, but carrying
the largest blocks in the file, since `analyzed-digest` listed every `.md` under
the spec directory with a 64-character hash each.

**047 resolved against this, and the ground has moved.** Its Resolved Questions
asked *"Should analyze write a per-spec `analysis.md` artifact instead,
mirroring `review.md`?"* and answered no: the gate reads the record's *counts*,
which are frontmatter, while an artifact would hold the findings' *content*,
which nothing reads. That reasoning was about **content**, and it still held —
findings routed through the inbox when this was written (058 has since moved
their deciding into the run that finds them), and this spec does not propose a
second triage surface. What 047 did not weigh is where the **record** lives. An artifact's
frontmatter is still frontmatter, so relocating the record costs the gate
nothing, and 047's own counter-argument — *"a new file in all 47 spec
directories"* — does not apply to the review record, whose file already exists
in all of them.

## Behavior

**One record, one home.** The review record lives in `review.md`'s frontmatter;
the analyze record lives in `analysis.md`'s frontmatter. `spec.md` carries
neither block. Every reader — the `done` gate, the status view, the audit
families — resolves the record from its owning artifact.

**The merge drops no field.** The two review records were not identical
supersets: `blocking` and `waivers` existed only in `spec.md`'s block, while
`diff-base`, `captured-issues`, and `skipped-passes` existed only in
`review.md`. The relocated record is the union of both sides, with the
timestamp keyed by meaning rather than name — `last-run` and `reviewed-at` were
always the same instant spelled two ways. 058 has since replaced
`captured-issues` with a `dispositions:` map, once the inbox stopped being a
destination.

**A reconciliation check becomes unnecessary.** With one copy of the record,
`check-review-agreement` and the audit family over it have no subject left to
compare and are retired by this change rather than re-pointed. That retirement
is the measure of the win: the check existed only because the fact had two
homes.

**The absent file is the never-run state.** A spec that has never been reviewed
has no `review.md`; one that has never been analyzed has no `analysis.md`. This
preserves what 047 called the record's absence being *itself* information, and
it removes the null-valued placeholder blocks a freshly created spec carried.
An artifact that exists but cannot be read is a different answer from one that
does not exist, and is reported as such rather than collapsed into never-run.

**The gate is unchanged in substance.** Every condition that blocked
`in-progress → done` before blocks after, on the same inputs, read from the new
location. This spec relocates state; it does not retune the gate.

**Staleness keeps one basis, with one asymmetry made explicit.** 047's subject
set — every `.md` under `specs/{feature}/`, `review.md` included — excluded the
spec's own `analyze:` block so that writing the record could not invalidate it.
That exclusion follows the record to `analysis.md`: the analyze run excludes its
own frontmatter record from the basis it digests. `review.md` stays *in* the
subject set, record and all, because a review superseding an earlier analyze run
is intended behavior (047's rendered `analyze` row reports superseded rather
than current in exactly that case).

**`spec.md` remains the spec's identity.** A directory carrying a record file
and no `spec.md` is not a feature: `read-spec` reports it missing, and no check
enumerates records independently of the spec they belong to. The orphaned
record is inert, exactly as an orphaned `review.md` is today — the relocation
gives the orphan more content, not a new failure mode, and adds no check for it.

**The CI gate is rewritten, not re-pointed.** The shipped template skips any
spec with no `review:` block, on the reasoning that such a spec predates the
feature. After this relocation that predicate is true of every spec, so the step
would pass for the whole corpus while checking nothing. Its replacement reads
the record from its new home and bounds the exempt set with a committed
high-water mark — 047's mechanism, which fails when the exempt set *grows* — so
no predicate in the step can become universally true.

**The schema is declared, not re-parked.** The constitution's spec-file
frontmatter table never listed `review:` or `analyze:` — the two largest blocks
in every `spec.md`, and the ones the `done` gate reads, sat under the
**Open-schema rule** as fields "permitted and ignored by uninterested
consumers." A gate-bearing record has no business being undeclared, and
[§runtime-boundary](../../framework/constitution.md#runtime-boundary) principle
4 settles the order: schema changes ship through the constitution and the
runtime updates to match. This change declares the relocated record in the
constitution rather than moving an undeclared block to a new address.

**Documentation moves with the record.** Twelve documents describe the blocks
today — the constitution, five command files, `docs/analyze.md` (whose stated
job is "the meaning of every field in the record"), `docs/slash-commands.md`,
`docs/shared-constitution.md`, `AGENTS.md`, `README.md`, and the CI template.
A relocation that updates the readers but not the documents leaves the corpus
describing a layout that no longer exists, which is the failure mode this
framework treats as drift rather than untidiness.

**Existing specs are migrated, not re-audited.** The records on disk move to
their new homes with their values intact. No record is invented for a spec that
was never analyzed, and no command is re-run to synthesize one — that would
assert a run nothing substantiates, the same objection 047 raised against
backfilling.

## Acceptance Criteria

- [x] AC1: `/{project}:analyze` writes `specs/{feature}/analysis.md`, whose frontmatter carries the run record 047 defines — `last-run`, `analyzed-against`, the three tier counts, `unexamined`, `unexamined-by-reason`, `analyzed-digest`, and the derived `blocking` — and whose body summarizes the run's findings.
- [x] AC2: `spec.md` frontmatter carries neither a `review:` nor an `analyze:` block, and the spec template ships without them, so a newly created spec opens with `status` and `dependencies` alone.
- [x] AC3: `/{project}:review` writes its run record to `review.md`'s frontmatter only, and no longer stamps `spec.md`.
- [x] AC4: `check-review-gate` reads the review record from `review.md` and the analyze record from `analysis.md`, and blocks `in-progress → done` on exactly the conditions it blocked on before the relocation — a differential check against the pre-relocation gate on the same inputs produces the same verdicts.
- [x] AC5: A spec with no `review.md` is reported as never-reviewed, and one with no `analysis.md` as never-analyzed; a present-but-unparseable artifact is reported as undeterminable, never as never-run.
- [x] AC6: `validate-frontmatter` validates each record against the artifact that owns it, and reports a `review:` or `analyze:` block still present in `spec.md` as a violation, so a stale copy cannot shadow the authoritative record.
- [x] AC7: The analyze staleness basis excludes `analysis.md`'s own frontmatter record, so an analyze run that has just written its record does not report itself stale; `review.md` remains in the subject set including its record.
- [x] AC8: `/{project}:status` and `dashboard` render the same review and analyze state they rendered before, sourced from the new locations.
- [x] AC9: `/{project}:audit`'s exempt-population family counts the same population from the relocated records, and its committed high-water mark is reconciled in the same change rather than left measuring a field that no longer exists.
- [x] AC10: A migration relocates every existing spec's records with values intact, invents no record for a spec that has none, and leaves `spec.md` with no residual block.
- [x] AC11: Full markdown lint passes across the migrated corpus.
- [x] AC12: The relocated review record carries every field from both former sides — `blocking` and `waivers` from the `spec.md` block, `diff-base`, `captured-issues`, and `skipped-passes` from `review.md`, and one timestamp for the pair spelled `last-run` / `reviewed-at` — so the merge drops no field. **Changed by 058**: the merge still dropped no field, and `captured-issues` has since been replaced by a `dispositions:` map, beside a new `decisions:` list.
- [x] AC13: `analysis.md`'s body carries no `- [ ]` checkbox items in any section, checked mechanically rather than by review, so the report cannot become a second triage surface.
- [x] AC14: Each analyze run overwrites `analysis.md` against a fixed section skeleton — Summary, hard failures, blocking findings, advisory findings, unexamined targets with their reasons, and captured issues — and never appends to a previous run's content. The sixth section is where the findings' text lands: the tier sections carry counts, because per-tier counts are all the writer receives, while the captured bullets record `family — message — path` without recording which tier produced them. **Superseded in part by 058**: the run still overwrites a fixed skeleton, but its sixth section is `## Fixed in this run`, and the tier sections list the findings, each with its disposition, because `write-analysis` now receives every finding with its tier.
- [x] AC15: `check-review-agreement` and the audit family built on it are removed, and the never-reviewed and never-analyzed cases its `single_sided` count distinguished remain distinguishable through AC5.
- [x] AC16: The change adds one `framework/migrations.toml` entry with its `framework/migrations/{id}.md` procedure file, and 027's audit family — which fails a convention removal carrying no registry entry — passes against it.
- [x] AC17: The shipped CI template step reads the record from `review.md` and `analysis.md` and bounds its exempt set with a committed high-water mark; no predicate in the step is satisfiable by every spec in a migrated corpus.
- [x] AC18: The migration procedure detects an adopter CI file still carrying the pre-relocation block-presence check and tells the adopter to re-copy the template, since `/{project}` does not install CI files.
- [x] AC19: Every analyze run writes `analysis.md` — including a clean run and an empty scope — so an absent file means never-analyzed rather than nothing-to-report, and the never-run signal AC5 depends on stays sound.
- [x] AC20: The migration is idempotent: re-running it over a partially migrated corpus completes the remainder and leaves already-migrated specs byte-identical, so an interrupted run is resumable rather than corrupting.
- [x] AC21: The constitution's **Frontmatter Schema** section declares the review and analyze records at their new homes — the owning artifact, each field, and its type — instead of leaving them under the Open-schema rule, and the runtime is updated to match that declaration rather than the reverse (§runtime-boundary principle 4).
- [x] AC22: `docs/analyze.md` documents the record field-by-field at `analysis.md` and describes the artifact's fixed skeleton; `docs/slash-commands.md`, `docs/shared-constitution.md`, and `README.md` carry no statement placing either record in `spec.md` frontmatter.
- [x] AC23: Every framework command file that names the blocks — `review.md`, `analyze.md`, `implement.md`, `audit.md`, `consolidate.md` — is updated in both its runtime steps and its markdown-only reference, so the two paths describe one location.
- [x] AC24: The generated per-agent command mirrors match their updated sources, and the command-parity audit passes.
- [x] AC25: `AGENTS.md` and the spec template's frontmatter guidance describe the new location.
- [x] AC26: A repo-wide search for the pre-relocation shape — a `review:` or `analyze:` block addressed as `spec.md` frontmatter — returns no hit outside this spec's own Motivation and Resolved Questions, so the documentation sweep is verified mechanically rather than asserted.

## Applicable Rules

- `QUAL-CLAIM-001` — two shapes in this spec are the rule applied: a never-analyzed answer given where the truthful answer is unreadable-record, and a CI step that reports a pass for a corpus it stopped examining.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Should `analysis.md` hold the findings' content as well as the record, or the record alone?** **Resolved: a prose report body, carrying no checkboxes.** `analysis.md` mirrors `review.md` — the frontmatter record plus a narrative body naming what the run examined, what it skipped and why, and the findings it produced. The line that keeps 047's objection satisfied is mechanical rather than editorial: **the body carries no `- [ ]` items**. A checkbox list is a triage queue, and a second triage queue is exactly what 047 rejected; the same findings stated in prose are a report. Routing stays with the inbox, which `/{project}:groom` walks and this artifact does not.
  - **Changed by `058-findings-route-at-discovery`.** Routing no longer goes through the inbox: each finding is fixed, routed, or discarded in the run that finds it, and the body lists each one beside its disposition. The no-checkbox line holds unchanged, so the report is still not a triage queue.
- **`analyze.md` or `analysis.md`?** **Resolved: `analysis.md`, with the frontmatter key left as `analyze:`.** The runtime already pairs `write-review` with `write-analysis` (`runtime/src/primitives/write_analysis.rs`), and `review.md` is the *noun* of its command rather than the verb — so the noun form is what both precedents point at, and it is the name 047 used when it discussed the artifact, which keeps that cross-reference legible. Renaming the frontmatter key to `analysis:` in the same change was considered and declined: it would widen a relocation into a key rename across every spec, for consistency that costs more than it returns.
- **Is `analysis.md` overwritten per run, or does it accumulate run history?** **Resolved: overwritten, against a fixed skeleton.** `write-review` "renders the fixed report skeleton" on every run (`runtime/src/primitives/write_review.rs:12`), so `review.md` always describes the current run and git carries the history. `analysis.md` takes the same shape, so the two artifacts stay symmetric and neither grows without bound on a command operators re-run freely.
- **Does the migration belong in this repo as a one-shot, or in the [027 bootstrap migration registry](../027-bootstrap-migration-registry/spec.md)?** **Resolved: one registry entry, covering both.** Removing the `review:` and `analyze:` blocks from `spec.md` is a convention removal, and 027's `/{project}:audit` family **fails** when a removed convention has no registry entry — so a one-shot is not an available shape, it is a shipped audit failure. The change adds an entry to `framework/migrations.toml` with its procedure file under `framework/migrations/{id}.md`; adopters receive it on their next `/{project}` run, tracked by `[migrations].last_applied`, and this repo — which runs the framework on itself — is migrated by the same entry rather than by a second code path that could disagree with it.
- **The adopter CI template grandfathers any spec with no `review:` block. After this relocation no spec has one — what replaces it, and how do adopters learn to re-copy?** **Resolved: a high-water mark, plus detection in the migration.** The template's step skips a spec when its frontmatter has no `review:` line, a predicate this relocation makes universally true — so the step would not go stale, it would go green everywhere. The replacement reads the record from `review.md` and `analysis.md` and bounds the exempt set with a committed high-water mark, the mechanism 047 already uses for the analyze exempt population: it fails when the exempt set grows, so it cannot be satisfied by emptying the subject. Failing closed instead was rejected — it would turn an upgrade into a corpus-wide CI failure for every adopter carrying legacy `done` specs — and dropping the step was rejected because it is the only one of 020's three mechanisms that catches a direct frontmatter edit. Because `/{project}` does not install CI files, the migration procedure detects the old shape in an adopter's copy and tells them to re-copy it.
- **What does a spec directory carrying a record file but no `spec.md` mean to the artifact checks?** **Resolved: `spec.md` is the spec's identity, and the orphan is inert.** The state is already reachable — `review.md` is a sibling file today, so deleting `spec.md` already leaves one behind. Every command keys off `spec.md`: `read-spec` reports it missing and `derive-dependencies` reports the spec absent. No check enumerates records independently of the spec they belong to, and this spec adds none; flagging orphans would mean a new family and a new enumeration path for a state nothing reaches in normal use, and having the migration delete them would make a relocation silently destructive.
