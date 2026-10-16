# 022 — Deterministic Runtime Tasks

Tasks derived from the [plan](plan.md). Complete in order. Each task is small enough to complete and verify in a single session; later tasks depend on earlier ones.

## 1. Implement scenario: [fetch-archive-bounds-the-whole-download](scenarios/fetch-archive-bounds-the-whole-download.md)

- [x] Implement the behavior described in `scenarios/fetch-archive-bounds-the-whole-download.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 2. Collapse AC31's superseded pair to the statement that holds now

- [x] Verify the post-058 half against the tree first: `dashboard` reports `inbox-standing`, `/{project}:status` renders the inbox row on every run in the four states, and `write-review` and `diff-cross-spec` no longer report it
- [x] Rewrite AC31 to that statement alone, with no **Superseded by 058** marker and no restatement of the retired `write-review`/`diff-cross-spec` contract

- **Done when**: AC31 states only what `dashboard` and `/{project}:status` do now, each claim checked against `runtime/src/primitives/dashboard.rs` and `framework/commands/status.md`.

## 3. Record why derive-routing-candidates does not derive a primitive's owning spec

- [x] Add a Resolved Question to `scenarios/specify-routes-before-scaffolding.md` (operator decision, 2026-10-16): plan Affected Files record which specs touched a file, not which owns it (`check_artifacts.rs` has seven claimants, fifteen primitive files have none because 022 claims the directory), and data-model headings name 44 of 67 primitives in one spec, 6 in two and 17 in none, `fetch-archive` among them
- [x] State that the owning-spec judgment stays at `routeInboxItem` and the operator, with the routing entry in `AGENTS.md` naming the three homes, and that a heading-based fallback would still answer 022 for `resolve-constitutions`, `resolve-references` and `enforce-manifest`

- **Done when**: the scenario records the decision and the measurements behind it, and no artifact describes deriving the owning spec as pending work.
