# 022 — Deterministic Runtime Tasks

Tasks derived from the [plan](plan.md). Complete in order. Each task is small enough to complete and verify in a single session; later tasks depend on earlier ones.

## 1. Rule-file review 2026-10-19: re-point rule citations the review changed

- [x] scenarios/the-constitutions-registry-validates-its-values.md cites CFG-ENV-008 (malformed value) for a malformed registry entry, not CFG-ENV-003 (missing required value)
- [x] scenarios/review-runtime-acceleration.md's degenerate-surfaces analogy names CFG-ENV-008
- [x] scenarios/fetch-archive-dns-rebinding.md's Context records that BE-INPUT-016 made address pinning a MUST on 2026-10-19 (it was low priority under BE-INPUT-007 alone)
- [x] plan.md's cargo audit step cites BE-DEPS-001 and the per-change scan of BE-DEPS-007

- **Done when**: Every rule citation in 022's artifacts matches the rule's current meaning in framework/rules/, and 022 is re-reviewed back to done.

## 2. Pin every redirect hop to its screened address (BE-INPUT-016)

- [x] Implement the behavior described in `scenarios/fetch-archive-dns-rebinding.md`
- [x] fetch-archive's connections, including every redirect hop, use only addresses that passed the internal-range screen (e.g. a screening dns::Resolve instead of a per-host resolve_to_addrs pin)
- [x] A test fails if a redirect host is resolved a second time without the screen

- **Done when**: No fetch-archive connection, initial or redirected, resolves a host without screening the addresses it connects to, and 022 is re-reviewed back to done.
