---
status: draft
dependencies: [050-constitution, 072-governing-documents-stay-within-one-read, 087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file]
cross-spec-impact: [050-constitution, 072-governing-documents-stay-within-one-read, 075-agents-md-holds-what-every-session-needs]
next-criterion: 5
---

# 089 — The pipeline's own cost is stated and bounded

A spec that adds a check, an audit-record field or a primitive states what a run of it costs and what it retires, and the governing documents and each command with its reference are held to size ceilings lower than one read, so the pipeline's own cost is stated where it grows and bounded where it is loaded.

## Motivation

- §design-principles held "Use what a task needs, and no more" ([050](../050-constitution/spec.md)'s AC26), which bound what a task used, not what the pipeline added. Spec 079's Motivation named a tie-breaker, that process cost never weighs in a recommendation (owner, 2026-10-27), which neither `AGENTS.md` nor the constitution held at `47cae941` (`grep -iE 'process cost|cost never'`); `AGENTS.md`'s Workflow had each option state the reopens and review runs it starts (owner, 2026-11-04), which bound a recommendation, not a design. Specs 078, 079, 081 and 082 each answered a convergence problem by adding primitives, and none retired one.
- `scripts/lint-governing-size.sh` held `AGENTS.md` and `framework/constitution.md` to 50,000 bytes each ([072](../072-governing-documents-stay-within-one-read/spec.md)), and both sat near it: 49,475 and 49,519 bytes at `6e9c96bf` (`wc -c`), 98,994 together, loaded through `CLAUDE.md` before any command ran. A cut with no lower ceiling regrows to the old one. No ceiling named a command and its reference: at the same commit `analyze`'s pair was 104,472 bytes (37,034 + 67,438) and `review`'s 101,788 (37,760 + 64,028).
- [087](../087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md) lets a project file keep an index and move its sections into parts that are measured as the file, so a cut to a lower ceiling can move a section rather than drop it.

## Behavior

- A spec that adds a check, an audit-record field or a primitive states in its body what a run of it costs, in a unit it can measure (model turns, bytes loaded, commits or wall time), and which existing check, field or primitive it retires, or why none.
- `AGENTS.md` and `framework/constitution.md` each sit under a ceiling below 50,000 bytes, and each command with its reference under a ceiling, checked on every commit by `scripts/lint-governing-size.sh`. Each is cut to fit by the routes 072 and 075 name, retiring, merging, or moving a rule to the check that enforces it or to the one command that applies it, or by moving a section into a part (087). Sizes are bytes by `wc -c`.

## Acceptance Criteria

- [ ] AC1: `framework/constitution.md` §design-principles states that a spec adding a check, an audit-record field or a primitive states what a run of it costs in a measurable unit, and which existing check, field or primitive it retires, or why none.
- [ ] AC2: `scripts/lint-governing-size.sh` holds `AGENTS.md` and `framework/constitution.md` to ceilings below 50,000 bytes and each `framework/commands/{name}.md` with its `framework/reference/{name}.md` to a ceiling, fails naming the file, its size, its ceiling and its fix when one is over, and a test proves each ceiling fails when exceeded and passes at it.
- [ ] AC4: 085, 086, 087 and 088 each state in their spec what a run of what they add costs, in a measurable unit, and what they retire, or why none, before any of them reaches `done`; each carries this spec in its `cross-spec-impact:` or a signpost back-linking it.
- [ ] AC3: Every file the script governs is at or under its ceiling at the commit that lowers the ceilings, and `scripts/audit/run-all.sh` passes there.

## Applicable Rules

- `QUAL-TEST-001` — each ceiling's test fails with the ceiling removed.

## Open Questions

- What are the new ceilings for `AGENTS.md` and `framework/constitution.md`?
- Is a command's ceiling held per file or over the command and its reference together, and at what size?

## Resolved Questions

- **The cost statement is a hard filter** (the agent's recommendation, adopted by the operator, 2026-11-25): a §design-principles bullet, so a design that states neither a run's cost nor what it retires is redesigned or deferred, as that section's preamble binds every principle in it. Rejected: an advisory prompt outside §design-principles, under which a design that skips it ships.
- **It binds every spec not yet `done` when it lands** (the agent's recommendation, adopted by the operator, 2026-11-25): 085, 086, 087 and 088 each state a run's cost and what they retire before their `done`, judged at each one's analysis. Rejected: binding only specs specified after it, under which the four open designs ship without the statement.
- **Sizes are measured in bytes alone** (the agent's recommendation, adopted by the operator, 2026-11-25): the ceilings bind bytes, and the governing documents' session-start load is their bytes, which `wc -c` measures the same way at any commit. Rejected: a before-and-after token comparison from Claude Code's session transcripts, which measures the same load through a tokenizer with session-to-session variance.
