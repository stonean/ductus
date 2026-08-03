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

- [x] Regenerate `.claude/commands/gov/analyze.md` and confirm the mirror matches its source
- [x] Run `npx markdownlint-cli2` across the repo
- [x] Run `scripts/lint-procedure-parseability.sh` so the new step parses as a single-primitive dispatch
- [x] Run `scripts/audit/run-all.sh` — check-zero catches a stale mirror, and Family 9 checks primitive promotion
- [x] Dogfood: run the capture against this repo's live findings and confirm a second run appends nothing, and that the dedup key preserves every distinct finding

- **Done when**: mirror in sync, markdown lint clean, procedure parseable, the 18-family audit exits 0, and a repeat capture run leaves `inbox.md` byte-identical.
