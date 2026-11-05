# 079 — Review and analysis work converges Research

Two read-only sweeps at `2abc19aa` that size the plan's correction and cut tasks. Both are floors or estimates, as each section says; re-measure before relying on a figure.

## Claims this work makes false (AC12)

Method: regexes per behavior over `framework/`, `docs/`, `README.md`, `runtime/src` doc comments and MCP text, `scripts/`, and each spec's `spec.md`, `plan.md`, `data-model.md` and scenarios, with the hits read in context. A unit is one line, paragraph or comment block holding a false statement. Resolved Questions, past-tense accounts, `path:line` citations and bare names are excluded. Tests, fixtures and mutation lists are not counted, though many assert the old behavior. A stale claim sharing no search term is missed, so spec counts are a floor.

Total: 203 claims, 197 distinct lines (six count under both window and scope), 111 outside specs and 92 in 20 specs.

| Behavior | Commands | References | Constitution | Docs, README | Runtime text | Specs: `spec.md`+`plan.md` / scenarios+data models |
| --- | --- | --- | --- | --- | --- | --- |
| Window (AC5) | 3 | 2 | 0 | 0 | 5 | 1 / 9 |
| Scope (AC3, AC6) | 2 | 2 | 0 | 0 | 5 | 2 / 13 |
| Fix-round paths (AC3, AC13) | 0 | 2 | 0 | 0 | 5 | 5 / 3 |
| `diff-cross-spec` (AC7) | 2 | 1 | 0 | 0 | 7 | 1 / 2 |
| Dispositions (AC2, AC17) | 9 | 11 | 2 | 7 | 5 | 19 / 5 |
| Reopen (AC4, AC15) | 3 | 3 | 4 | 1 | 5 | 10 / 7 |
| Gate check 12 (AC14) | 0 | 2 | 0 | 0 | 3 | 1 / 3 |
| Canonical surface (AC9) | 0 | 1 | 0 | 0 | 19 | 5 / 6 |

| Spec | Status | `spec.md`+`plan.md` | Scenarios+data model |
| --- | --- | --- | --- |
| 000 | in-progress | 1 | 0 |
| 011 | done | 1 | 0 |
| 016 | in-progress | 2 | 0 |
| 017 | in-progress | 2 | 0 |
| 020 | in-progress | 0 | 3 |
| 022 | in-progress | 1 | 8 |
| 023 | done | 0 | 1 |
| 041 | in-progress | 1 | 1 |
| 045 | in-progress | 0 | 1 |
| 046 | done | 2 | 0 |
| 047 | in-progress | 2 | 0 |
| 050 | in-progress | 1 | 6 |
| 054 | in-progress | 2 | 0 |
| 058 | in-progress | 16 | 2 |
| 059 | done | 1 | 0 |
| 063 | in-progress | 3 | 1 |
| 066 | in-progress | 2 | 19 |
| 075 | in-progress | 1 | 0 |
| 076 | done | 0 | 1 |
| 078 | in-progress | 6 | 5 |

Of the `done` specs, correcting 023's scenario (`scenarios/living-specs.md`) and 076's data model (`data-model.md`) reopens them and owes a review; correcting 059's ticked AC3 changes a criterion, so it reopens and owes an analysis; 011 and 046 stay `done` under AC15.

Separately, 112 lines in 30 specs name the markdown-only path, by a text search for `two paths`, `walk the same prose`, `neither wraps`, `markdown-only path`, `runtime is unavailable` and `when no runtime`; 17 are `done`, 7 with the lines in a scenario or data model (010, 013, 019, 023, 030, 032, 040). That is a ceiling on what AC21 falsifies, since most are procedural lines to read, and the search was not read in context.

Most-read targets: `framework/commands/review.md:127`, `framework/reference/review.md:472` and `:589`, `framework/commands/implement.md:85`, `framework/constitution.md:160`, `:162`, `:295` and `:296`, `runtime/src/mcp/server.rs:638`, `specs/066-audit-records-and-the-completion-gate/data-model.md:42` and `scenarios/review-scope-union.md:37`, and `specs/058-findings-route-at-discovery/spec.md:480`. Commit line numbers rot; grep the quoted claim.

## Prose to cut (AC18, AC21)

Method: each section read and given a kept fraction; cut = measured paragraph bytes times one minus the fraction, with 75% as the planning yield. No section was trial-rewritten. Strict keeps every rule a walker needs; pointer also drops a primitive's rules, which AC21 allows.

| File | Bytes at `2abc19aa` | Must lose | Strict cut | Pointer cut | At 75% of pointer |
| --- | --- | --- | --- | --- | --- |
| `framework/reference/review.md` | 64,606 | 14,606 | 23,460 | 33,240 | About 39,700 |
| `framework/reference/analyze.md` | 79,745 | 29,745 | 29,650 | 42,400 | About 47,900 |

Largest sections, with pointer cuts: in `review.md`, Write `review.md` (11,520, cut 7,000), Resolve target and scope (10,127, cut 7,500), Fix-round loop (6,190, cut 3,650); in `analyze.md`, Rules (8,914, cut 2,870), Project-level consistency (8,306, cut 4,720), Acceptance-criterion path existence (7,361, cut 5,420), Finding dispositions (7,256, cut 3,720).

Elsewhere in commands and references: `> **For agent runtimes**` is 7,464 bytes over 15 hits, saving about 6,500, and `**Where a named section lives.**` 11,104 bytes over 26 hits, saving about 6,000. Restatement in the five largest commands is about 23,900 bytes, of which about 16,700 is removable (sampled). `amend`, `help`, `link` and `target` have no reference file, so their no-server text stays.

Every heading keeps its exact text, since a command step, `framework/bootstrap/ductus.md` or `AGENTS.md` cites it: `runtime/src/primitives/resolve_anchor.rs` holds the corpus test, `scripts/audit/template-alignment.sh` hardcodes nine blocking headings, and `scripts/audit/marker-list-parity.sh` matches three sentences in `analyze.md`, which must change with them. `framework/bootstrap/ductus.md` cites `review.md`'s §Resolve target and scope item 5 by number, so that list must not renumber.

The ten `check-artifacts` families hold about 27,000 reference bytes against a 1,333-byte description. They are dropped, not moved.

Constitution headroom: 64 bytes at `2abc19aa`, so AC2, AC14, AC15, AC17, AC21 and AC9's table row must be net-negative together.
