---
next-task: 65
---

# 022 — Deterministic Runtime Tasks

Tasks derived from the [plan](plan.md). Complete in order. Each task is small enough to complete and verify in a single session; later tasks depend on earlier ones.

## 60. The 0.60.0 release notes state what the pre-tag audit of the unpushed bump found missing or false (owner decision, 2026-11-19: fix everything the audit found)

- [x] Entries for spec 080's adopter-visible changes, which the notes lack: the MCP tools `render-status`, `read-section`, `render-help` and `merge-status-hook` are new since `ductus-v0.59.11`; `merge-status-hook` installs the Claude Code status and help hooks; and `dashboard` no longer returns `rendered-markdown`, a breaking change to its result. Correct the existing sentence that says `dashboard` names `rendered-markdown`
- [x] An entry saying `line-number-citations` is a new `check-artifacts` family that blocks at every status
- [x] An entry saying a `.ductus/config.toml` or session file that does not parse, or an invalid `[paths] specs-root`, halts every reader where 0.59.11 warned and used the default, and correct the sentence in the path-argument entry that says a malformed config is warned about
- [x] Remove the Added and the Removed entry for `check-review-freshness`, a tool added and retired inside this release that no release served
- [x] Give the `substitute-seed-placeholders` migration an entry of its own, as the other five 0.60.0 migrations have
- [x] One lockfile entry stating the net versions from 0.59.11 to the release head and naming the advisory the `pulldown-cmark` update addressed, since the recorded steps stop at 2026-10-29
- [x] State the command and reference byte totals at the release head, not mid-release, in the entry on what a command's reference holds; state the shipped rule total; and qualify the `prune-project` entry's mention of a configure block for an agent this release drops
- [x] Ask the owner, entry by entry, which entries that make a call refuse or rename a wire value are marked BREAKING, since the audit found about twelve unmarked; the owner ruled the seed narrowing not breaking on 2026-11-19

- **Done when**: each item below is an entry or a correction under `[Unreleased]` in `runtime/CHANGELOG.md`, checked against the code at the head it describes, and the entry and BREAKING counts any release summary states are measured after the last edit

## 61. The remaining hand-rolled fence toggles close a fence as `SkipScanner` does (found while implementing task 54)

- [x] `extract_section_body` in `runtime/src/interpreter/payload.rs`
- [x] `top_level_literal_markers` in `runtime/src/parser/mod.rs`
- [x] the in-fence toggle in `runtime/src/primitives/spec_links.rs`

- **Done when**: each of the three sites reads a fence by the length-aware rule scenario `skipscanner-inline-code-exemption` states, a test per site fails without it, and the release binary before and after gives identical results for every spec, taken on one committed tree

## 62. Routing ranks by claimed path

- [x] Implement the behavior described in `scenarios/routing-ranks-by-claimed-path.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 63. Disposition out-of-spec finding: 080's groom-derives-routing-candidates scenario lists three routing sources

- [x] specs/080-deterministic-runtime-efficiency/scenarios/groom-derives-routing-candidates.md — its Context says derive-routing-candidates derives homes from "a runtime-owning spec, a rule file, a spec sharing vocabulary", and task 62 added a fourth source, path-claim, ranked first; 080 is done, so editing the scenario reopens it — fixed: the Context now names the sources by pointer to `ductus derive-routing-candidates --help`, and 080 reopens `done → in-progress` for it (the agent's recommendation, adopted by the operator, 2026-11-24)

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task

## 64. routeInboxItem moves unclaimed-paths past the read size

- [ ] `build_route_inbox_item_request` in `runtime/src/interpreter/payload.rs` passes `unclaimed-paths` to `bound_lists` with `available-specs` and `candidates`, and a test past the bound asserts that `unclaimed-paths` moved to the lists file and is absent inline (found by 022's analysis of 2026-11-24, BE-PAYLOAD-001)

- **Done when**: the data model's `routeInboxItem` row holds for all three lists, and the test fails without the change
