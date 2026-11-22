---
section: "Waivers"
---

# Waiver expiry

## Context

A MUST violation found by `/ductus:review` has been waived via `--waive <rule-id> --reason "<text>"`. The waiver record is anchored to a specific `(rule, file, site)` triple in the review record's `waivers` frontmatter list. On subsequent `/ductus:review` runs the system must decide whether each waiver still applies — the rule, the file, the site and the offending code may all have changed since the waiver was recorded.

Anchoring on `(rule, file, site)` is intentional. A waiver is a statement about *this specific spot* in the codebase being an acceptable violation; it is not a project-wide pardon for the rule, nor a file-wide one. When the anchor moves or disappears, the justification no longer attaches to a specific location, so the waiver expires and the framework returns to its default position of blocking on MUST violations. A file alone is too wide a spot: anchored on `(rule, file)`, as through 0.62.0, a waiver decided for one finding also covered every other finding of its rule in that file, unrelated code and code added later included, under a reason describing none of them, and the record could not say which findings it was decided for. An adopter reported exactly that.

A violation's deferral under `decisions:` ([076](../../076-waived-and-deferred-findings-are-distinct-dispositions/spec.md)) is anchored the same way, so every rule below holds for it too, a drop being the deferral's expiry.

A finding's **site** is the narrowest named unit enclosing it that a pass can name again on a later run — a function or method, a type, a configuration key path, a markdown heading — spelled as the file spells it; a finding outside any named unit names `(top level)`. A line number is never a site.

## Behavior

For each waiver in the review record's `waivers` list at the start of every `/ductus:review` run, before counting findings into `must-violations` (`review.waivers` in spec frontmatter until `057-analyze-artifact-and-record-relocation` moved the record into `review.md`). Each rule applies per anchor: a waiver listing several anchors ([waiver-file-lists](waiver-file-lists.md)) is that many anchors, so where a rule below drops the waiver, it drops that anchor, and the waiver goes with its last one.

Each anchor also records how many findings it **covers**: the number firing at it when the waiver was decided, never raised by a later write, and lowered to the number a run applied it to when that is fewer.

1. **File still exists at the anchored path, the rule still fires at the anchored site, and no more findings fire there than the anchor covers** → the waiver applies. The finding is recorded under `## Waived findings` in `review.md` with the waiver's `reason`, and excluded from the `must-violations` count.
2. **File no longer exists at the anchored path** (renamed, deleted, or moved) → the waiver is dropped from the list on the next write of `review.md`. The framework does not chase renames — the operator explicitly anchored to that path, and a path change is a meaningful event worth re-evaluating.
3. **Code has moved within its site, or the site has moved within the file** → the waiver applies. Line numbers are not part of the anchor; rule + file + site is the contract. Code moved to another site is rule 4 at the old site and rule 5 at the new one.
4. **Rule no longer fires at the anchored site** (offending code was fixed or moved out of that site) → the waiver is dropped from the list on the next write of `review.md`. There is nothing to waive at that location. A run whose scope does not hold the file did not read it, so it retains the waiver instead (079's AC24).
5. **The same rule fires at a *different* file, or at a *different* site in the same file** → the waiver does NOT extend to the new location. A waiver is a per-location decision. If the violation at the new site is also intentional, the operator records a separate `--waive` for it.

   **Exception: a departure waiver.** A waiver `write-review` derives from a spec's departure entry (`departure: true`) does extend to a new location: the entry is the spec's decision, so each run anchors it at every file in the spec's own review scope where its rule fires, covering every site there, and a site or file added later is covered with no second decision. Each anchor still applies, is dropped or is retained under rules 1–4, never under 6 or 7, and every anchor expires at once when its entry is gone ([073 — Decided rule departures are recorded](../../073-decided-rule-departures-are-recorded/spec.md)).
6. **More findings fire at the anchored site than the anchor covers** → the waiver covers none of them and is dropped from that anchor. A finding beyond the count is one nobody judged, and nothing tells it apart from the ones that were, so the operator decides afresh with every finding there in view.
7. **The anchor names no site** — an entry recorded before sites → it covers no finding and is dropped on its spec's next review, since nothing says which findings it was decided for.

When step 2, 4, 6 or 7 applies and the same rule still fires *anywhere in scope* (including at the anchor itself under steps 6 and 7, and in the same file when the anchor was lost via path rename), the underlying finding re-counts toward `must-violations` and the record's `blocking` flips back to `true` if it was previously `false`. The spec returns to the blocking state until the violation is fixed, a fresh waiver is recorded, or a deferral releases the violation (spec 076).

Every drop emits a one-line notice on stdout so the operator notices the lost coverage: `waiver expired: rule {rule-id} at {file} in {site} ({reason})`; under step 6, `waiver expired: rule {rule-id} at {file} in {site} — {n} findings fire where it covers {m} ({reason})`; under step 7, `waiver expired: rule {rule-id} at {file} — recorded before sites ({reason})`, so the operator re-decides from the reason given. Silent expiry would let waivers quietly evaporate — the notice is the point.

A finding whose pass names no site still counts, and no stored waiver or deferral covers it. `--waive` and `--defer` refuse it, naming the finding, because an anchor without a site would cover nothing (step 7).

## Edge Cases

- **File renamed to a path also covered by an existing waiver for the same rule** — the renamed file does not inherit the existing waiver. Each waiver is independent; the framework does not merge waivers across path collisions. The pre-rename waiver expires (path no longer exists); the new path's waiver continues to apply only to its original anchor.
- **Site renamed** (a function renamed, a heading reworded) — as a file rename: the old site no longer fires (step 4), and the finding at the new name is at a different site (step 5).
- **Two findings at one site when decided** — the waiver covers both, so its anchor covers 2. When one is fixed, the next run lowers the count to 1, and a later second finding there is step 6.
- **One finding fixed and an unrelated one added at the same site between two reviews** — the count holds, so the anchor covers the new finding. This is the decided residual (Resolved Questions).
- **Rule removed from the framework entirely** — every waiver referencing the removed rule expires on the next run. The expiry notice still emits for visibility; subsequent runs no longer process the rule.
- **Rule renamed (ID changed)** — a new ID is a different rule by `ductus`'s rules-tier conventions (IDs are permanent per `specs/008-security-rules/data-model.md`). Existing waivers referencing the old ID expire; new waivers must be recorded against the new ID if the violation persists.
- **Waiver record malformed** (missing a required field) — `/ductus:review` skips the malformed waiver, emits a one-line warning naming the offending entry, and proceeds. The malformed entry is not auto-removed; the operator must clean it up to prevent the warning from recurring. An entry recorded before sites is not malformed: step 7 drops it.
- **Multiple waivers for the same `(rule, file, site)` anchor** — only the first applies; duplicates emit a warning on the run that observes them and are not auto-pruned. The framework treats duplicates as operator-authored noise to investigate, not silent state to clean up.

## Open Questions

*None.*

## Resolved Questions

- **What does a stored waiver or violation deferral anchor to?** `(rule, file, site)`, matched exactly by the runtime: the agent's recommendation, adopted by the operator, 2026-11-22. It keeps waiver arithmetic deterministic and working under `ductus exec`, where the passes' findings bind straight to `process-waivers`, and a site named differently on a later run expires the waiver with its notice, so drift costs one repeated decision and is never silent. Rejected: storing the finding's description for the host to match, as observations are, because a false match absorbs a finding silently and under `ductus exec`, with no host to match, no stored waiver would apply; and keeping `(rule, file)` with a count of findings, because a fixed finding replaced by an unrelated one keeps the count and is absorbed silently.
- **What does an entry recorded before sites cover?** Nothing: it expires on its spec's next review with a notice quoting its reason, and its findings re-count (step 7): the agent's recommendation, adopted by the operator, 2026-11-22. At `0400e970` this repository held 8 such anchors in 7 review records, 3 waivers and 5 violation deferrals, counted over each `specs/*/review.md` frontmatter. Rejected: keeping a site-less anchor over its whole file, which leaves the reported defect live for every existing record; and adopting the one finding's site when exactly one fires, which absorbs, once and silently, an unrelated finding that replaced the original.
- **Two findings of one rule at one site?** A count guard: each anchor records how many findings it covers, lowered and never raised, and covers none when more fire (step 6): the agent's recommendation, adopted by the operator, 2026-11-22. Its residual, one finding fixed and an unrelated one added at the same site between two reviews, is accepted with the option: a site is the finest unit a pass can name that survives code movement (step 3), and the count is the finest a deterministic match can read without one. Rejected: waiving the residual outright, which leaves every later finding of the rule at a dispositioned site absorbed.
