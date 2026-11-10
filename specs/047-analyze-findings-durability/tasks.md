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

## 4. Surface and enforce analyze-record freshness

- [x] Implement the behavior described in `scenarios/analyze-record-freshness.md`

- **Done when**: `/{project}:review` renders the three-state `analyze` row from a working-tree comparison, `check-review-gate` blocks a stale record naming the changed paths, and both surfaces agree on the subject set.

## 5. Make the analyze record say what it examined

- [x] Add `analyzed-digest` to `AnalyzeBlock` and `write-analysis`: a per-path digest of the subject set as read from disk, excluding the spec's own `analyze:` block, with unreadable subjects recorded as unreadable rather than digested as empty
- [x] Replace the sha-diff staleness basis with a digest comparison, retiring `Compare`'s two modes and the working-tree/committed split
- [x] Keep `analyzed-against` as provenance and read it only for the mechanical-sweep rename exemption, reporting candidates when its trees are unavailable
- [x] Report a digest-less record as undeterminable — not current, not stale, and not a sha-diff fallback
- [x] Prove the false-positive path is gone: a record written against a dirty tree must not stale when that same content is committed
- [x] Re-run `/{project}:review` so the retired `QUAL-CLAIM-001` finding leaves the report by being fixed rather than dispositioned

- **Done when**: staleness is a digest comparison, the row and the gate return the same answer before and after a commit, a record predating the digest reads undeterminable, and committing content an analysis already read does not block.

## 6. Correct the advisory tier's promotion-criteria claim

- [x] scenarios/analyze-run-durability.md: the advisory tier is advisory by design — some checks carry published promotion criteria, and the plan record and un-folded branch specs stay advisory for good

- **Done when**: the scenario says what the tier is, matching analyze.md and the constitution, and 047's review and analyze record this reopen

## 7. Return to done after AC2's work-item correction

- [x] AC2 listed a finding's disposition as "the fixed file, the artifact it was deferred to, or the waiver reason", which 076's deferral to an external work item alone (rendered "deferred to work item `X`") made incomplete. Found by 077's review on 2026-11-03. — owner's call: reopen 047 and correct AC2; or waive — reopened: AC2 reads "the artifact or work item it was deferred to" (owner, 2026-11-03, option a)
It owes `/ductus:review` and `/ductus:analyze` over the correction, its ticked criteria re-verified against the tree before they run, then the completion gate and the owner's approval of `done`, which run once no box in this file is open (spec 079).

- **Done when**: 047's review and analysis are recorded and not stale, AC2 holds against the tree, and the owner has approved `done`.

## 8. What the criteria verification found at `60c28d26`, corrected before the records

- [x] Criteria verified at `60c28d26` by one read-only agent reading the source, with no test run (262 s), and AC8 by the operator (`scripts/gen-claude-commands.sh --check` exit 0; `markdownlint-cli2` 0 issues in 875 files): AC1–AC4, AC6–AC8 and AC11–AC13 hold; AC9's, AC14's and AC15's annotations are corrected, superseded by `29ec6cc2` (079's refusal while a task is open), `2a4779c0` (057's whole-file exclusion of `analysis.md`) and `6a08a7ad` (066's normalized criterion ticks); AC5's names what the record does count; AC10's passing-verdict clause is annotated never held as written (`97ca0294`)
- [x] Body, scenarios and plan, corrected by tense or fact (owner's delegation, 2026-11-10): the spec header's two "unchanged" claims; `analyze-run-durability.md`'s Behavior note, record rule, added fields, gate blocks, re-analysis and newline edge cases, and 076's disposition names; `analyze-record-freshness.md`'s whole-`spec.md` digest and its gate order; the plan's capture design, annotated as reversed by 058
- [x] Runtime: the doc comment on `check_review_gate.rs`'s `advisory_findings_and_unexamined_targets_do_not_block` says the counts ride an `analyze-findings` block, never a passing verdict
- [x] Waived: `framework/reference/analyze.md` `## Record fields` says the digest excludes "`analysis.md`'s own record", which reads true of the file as the record, so it takes no edit

- **Done when**: every criterion holds or carries a true annotation, and no contract or body passage is false about the tree
