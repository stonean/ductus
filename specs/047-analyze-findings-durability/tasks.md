# 047 — Analyze Findings Durability Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Amend the constitution's auto-capture contract

- [x] Widen §Automatic issue capture's opening scope from issues an agent notices incidentally to include the findings a command produces as its primary output
- [x] Add `/{project}:analyze` to the **Surface at completion** bullet alongside `/{project}:implement` and `/{project}:review`
- [x] Verify the four existing bullets are unchanged in substance

- **Done when**: §Automatic issue capture names `/{project}:analyze` as a surfacing gate and its scope covers a findings-producing command, with no other bullet's meaning altered.

## 2. Add the capture step to `analyze.md`

- [x] Insert the capture step before the render step, dispatching `append-inbox` once per surviving finding with a `dedup-prefix` of `{category}: {family} — {message}` (the message is load-bearing: a finding's `path` is the citing artifact, not the missing subject)
- [x] Exclude findings resolved by `--fix` in the same run, and exclude informational entries (`skipped` unexamined targets, cross-service reference unknowns)
- [x] Renumber the render step and grep the command body for any cross-reference to the old step number
- [x] Document the same behavior in the markdown-only reference for the runtime-less path

- **Done when**: `analyze.md` captures every surviving finding before rendering, the render step is host-responsibility prose with no primitive dispatched after it, and no stale step cross-reference remains.

## 3. Verify and regenerate

- [x] Regenerate `.claude/commands/ductus/analyze.md` and confirm the mirror matches its source
- [x] Run `npx markdownlint-cli2` across the repo
- [x] Run `scripts/lint-procedure-parseability.sh` so the new step parses as a single-primitive dispatch
- [x] Run `scripts/audit/run-all.sh` — check-zero catches a stale mirror, and Family 9 checks primitive promotion
- [x] Dogfood: run the capture against this repo's live findings and confirm a second run appends nothing, and that the dedup key preserves every distinct finding

- **Done when**: mirror in sync, markdown lint clean, procedure parseable, the 18-family audit exits 0, and a repeat capture run leaves `inbox.md` byte-identical.

### Analyze-run durability — the record, the gate, the bounded exemption

Implements `scenarios/analyze-run-durability.md`. Findings outlived the session; the run did not, so the pipeline's second gate could not be enforced at all.

- [x] Implement the behavior described in `scenarios/analyze-run-durability.md`
- [x] Add the `analyze:` frontmatter block (`AnalyzeBlock`) and the `write-analysis` primitive that writes it, registered at all five sites
- [x] Write the record on **every** run, including a clean one and an empty scope — the record's purpose is that its absence means something
- [x] Record `unexamined` alongside the finding counts, so a clean record cannot be read as a fully-examined one
- [x] Record `advisory` without gating on it, and state why the asymmetry with `review:`'s SHOULD handling is deliberate rather than an omission
- [x] Refuse to write a record into a spec whose frontmatter does not parse
- [x] Extend `check-review-gate` with `not-analyzed` and `analyze-findings`, ordered after every `review:` check, with no grandfather clause at the gate
- [x] Add the `analyze-state-drift` family to `check-artifacts` (eight residual families → nine), grandfathered for specs predating the record
- [x] Make the exemption bounded rather than silent: `/{project}:audit` Family 37 holds the exempt population against a committed high-water mark, failing when it grows and naming a loose baseline when it shrinks
- [x] Restate `/{project}:analyze`'s read-only contract around the subject/observation line, and add the record-writing step ordered before the render
- [x] Ship the block in the spec template so new specs carry it from the start
- [x] Prove each gate reason and each drift case fails before keeping it, and verify the whole path through the built release binary rather than the MCP tools

- **Done when**: a spec with no analyze record cannot reach `done`, a clean record carries its unexamined count, advisory findings never gate, the grandfathered set is counted against a baseline that cannot silently grow, and every one of those was demonstrated failing first.
