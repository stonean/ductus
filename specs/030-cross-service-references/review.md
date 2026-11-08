---
spec: 030-cross-service-references
last-run: 2026-11-08T01:17:41Z
reviewed-against: 7bdd9c93319b5860265b573f45f18319186fc62c
diff-base: 8339e0f67c811f424936f698031bf4f0f6887bf2
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 19
scope: 23
excluded: 1
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: c7a53fc4930ca1ca6b53abc86275574ab63e0d0195bb6211bc0e048eaf478e7e
  scenarios/referenced-service-spec-root.md: c76e9244f75881073a432932610036a6f425830df413a593c1e50c65c1b7dd13
blocking: false
dispositions:
  fixed: 4
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: "other — round 1's re-read (NOTE): under `--all` step 13 runs per feature, so one duplicated repo is reported once per spec whose references resolve through the registry — `framework/commands/analyze.md`"
    outcome: waived
    reason: each spec's analysis is its own record, and the duplicate bears on every spec resolving a reference through that registry; the finding is advisory, and one registry edit clears every record
    decided-at: 2026-11-08T00:25:23Z
    decided-by: rule
  - key: "other — round 1's re-read (NOTE): step 13 is skipped for a spec declaring no references, so a duplicated repo is reported only through a spec that has some — `framework/commands/analyze.md`"
    outcome: waived
    reason: with no reference nothing resolves through either alias, so the silent choice the finding names cannot occur; the data model and docs state what resolution reports, which holds
    decided-at: 2026-11-08T00:25:23Z
    decided-by: rule
  - key: "other — round 2's re-read (NOTE): `normalize_repo` trims every trailing `/` before one `.git`, so `a trailing /`, as `link.md`, the data model, the description and the reference say, under-describes a URL ending `//` — `framework/commands/link.md`"
    outcome: waived
    reason: contrived input; `normalize_repo`'s doc comment states it exactly, and editing four copies for a `//` URL would start another round for no registry a person writes
    decided-at: 2026-11-08T00:25:23Z
    decided-by: rule
  - key: "other — round 1's re-read (NOTE): `/link` rejects only an empty path, while `Services::validate` reads a whitespace-only one as empty too, so such a path passes `/link` and then halts every runtime reader — `framework/commands/link.md`"
    outcome: waived
    reason: "contrived input, and not silent: the next command that reads the registry halts naming the alias, the field and the file"
    decided-at: 2026-11-08T01:17:41Z
    decided-by: rule
  - key: "other — round 1's re-read (NOTE): `/link --list` reads the registry with host file tools and would list a hand-edited empty path as resolving, since it resolves to this repository — `framework/commands/link.md`"
    outcome: waived
    reason: only a hand edit writes an empty path now that `/link` rejects one, and every runtime reader halts naming it on the next command, so the listing's line cannot stand unnoticed
    decided-at: 2026-11-08T01:17:41Z
    decided-by: rule
---

# Review — 030-cross-service-references

## Summary

Window from `8339e0f6` (`base-source: last-review`, this reopening's first review), `review-owed: true`, owed because task 15's code (`3cfe8be7`) landed after that review, which the analysis (`85c84c08`) deferred three blocking findings to. The passes' scope was 17 entries, each a file, with 1 path excluded as another spec's (`specs/022-deterministic-runtime/tasks.md`, `7bdd9c93`); this record's `scope` adds the 6 uncommitted paths this run's fixes wrote. Examined 19: every one of the 17, read as the window's diff (`git diff 8339e0f6 HEAD`) per file, the whole of `specs/030-cross-service-references/analysis.md`, and the two command sources this run edited, `framework/commands/analyze.md` (step 13) and `framework/commands/link.md` (Validation). Not opened, and not counted: `.claude/commands/ductus/analyze.md`, `.claude/commands/ductus/link.md`, `.pi/prompts/ductus-analyze.md` and `.pi/prompts/ductus-link.md`, generated from those two sources by `gen-claude-commands.sh`, re-run, and diffed against their sources by the round's re-read. No finding under the 12 rule files: CFG-ENV-003, CFG-ENV-008 and BE-PAYLOAD-001 now hold in the code, the error message escapes the alias (BE-INPUT-011), the validation is pure value inspection, and six mutants are killed (QUAL-TEST-001). Four chores fixed by the rule. One fix round from `start-fix-rounds`' base `f0b6e256`, budget 2, re-read whole and clean with two NOTEs, both waived; the next round from `3b52d78e` was empty. The whole `cargo test --locked --no-fail-fast` on the round's tree passed 2,761 with 0 failed, with procedure parseability, tool coverage and markdownlint clean. Of the stored decisions, three on files the passes' scope did not hold are retained, and one on `services.rs` expired: checked against the file, its issue no longer stands, since `Services::from_toml_str` rejects an empty `repo` before `duplicate_repos` can group it and every reader goes through that parser. The reopening's first review recorded no gating finding, so this second one is over AC1's bound by `measure-reopenings`' rule; the cause is that a MUST finding at analysis needing code costs a second review under the review-then-analysis order.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

*None.*

## Observations

- convention — `/analyze` step 13 did not send the host to `lists-file` when `resolve-references`' result takes the file form, so a bounded result's absent lists could read as no references; it now says to read both lists from that file, as steps 1 and 18 say for their primitives — `framework/commands/analyze.md` — **fixed** (decided by the rule)
- convention — `/link`'s path validation recorded any value as written, so an empty path, which resolves to this repository and passes its not-checked-out warning, wrote an entry every reader of the registry now halts on; it now rejects an empty path — `framework/commands/link.md` — **fixed** (decided by the rule)
- convention — the plan's D1 said `config_keys.rs` is what both readers call, where since task 15 the registry has one reader, `load_services`; corrected — `specs/030-cross-service-references/plan.md` — **fixed** (decided by the rule)
- convention — `load_registry` kept a block around the checkout's resolution that held its `is_empty` branch before task 15; flattened — `runtime/src/primitives/derive_references.rs` — **fixed** (decided by the rule)
- other — round 1's re-read (NOTE): `/link` rejects only an empty path, while `Services::validate` reads a whitespace-only one as empty too, so such a path passes `/link` and then halts every runtime reader — `framework/commands/link.md` — **waived**: contrived input, and not silent: the next command that reads the registry halts naming the alias, the field and the file (decided by the rule)
- other — round 1's re-read (NOTE): `/link --list` reads the registry with host file tools and would list a hand-edited empty path as resolving, since it resolves to this repository — `framework/commands/link.md` — **waived**: only a hand edit writes an empty path now that `/link` rejects one, and every runtime reader halts naming it on the next command, so the listing's line cannot stand unnoticed (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `specs/022-deterministic-runtime/tasks.md` — 022-deterministic-runtime

## Unexamined governance

*None.*
