# `scripts/audit/`

Per-family check scripts for `/audit`. See [spec 026](../../specs/026-framework-self-audit/spec.md) and the [026 plan](../../specs/026-framework-self-audit/plan.md) for the design.

## Contract

Every script in this directory follows the same contract:

- **Output.** Findings written to stdout, one per line, in the format `FAMILY | LOCATION | MESSAGE | SUGGESTED-FIX` (pipe-separated, columns aligned for readability when the runtime renders the aggregated output).
- **Exit code.** `0` when no findings; `1` when any finding is present. Aggregated by `/audit` via logical OR — any family with findings makes the whole audit fail.
- **Read-only.** No file modifications. Scripts may write to `$TMPDIR` for intermediate computation but must not touch the working tree.
- **Idempotent.** Same inputs produce identical output across runs.
- **Self-contained.** Each script can be invoked directly (`bash scripts/audit/{family}.sh`) from any working directory, without orchestration — useful when triaging a specific check.

The contract's mechanics live in `lib.sh`, which every script sources before doing any work:

```bash
set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family {family}
```

Sourcing resolves the repo root and `cd`s into it, seeds the `drift` accumulator, and defines `emit LOCATION MESSAGE SUGGESTED-FIX` — which renders the finding line above and sets `drift=1`. `audit_family` names the leading column. A script ends with `exit "$drift"`.

Three properties this buys, each of which the shape above is load-bearing for:

- **Direct invocation keeps working.** The lib path is derived from the script's own location, not the caller's working directory.
- **The finding shape is defined once.** A change to the output format is a one-file edit. Where a check computes findings in python (`installer-registry-parity.sh`, `migration-coverage.sh`), python prints tab-separated `location<TAB>message<TAB>fix` records and the shell renders them through `emit` — the pipe-separated shape never appears in the python.
- **A missing `lib.sh` fails closed.** `|| exit 1` on the source line makes the script exit non-zero instead of continuing without `emit` (which would otherwise let a stub family report clean).

## Scripts

- `lib.sh` — shared boilerplate (repo root, `drift`, `emit`). Sourced by every script here; not a check itself.
- `check-zero.sh` — generator/lint precondition pass. Run before family checks; halts `/audit` on failure to avoid misleading findings against known-stale generator output.
- `cross-doc-consistency.sh` — Family 1.
- `manifest-parity.sh` — Family 2.
- `placeholder-roundtrip.sh` — Family 4. (Family 3, registry equivalence, was retired with the workflows feature — spec 043; family numbers are stable identifiers, so the gap stands.)
- `template-alignment.sh` — Family 5.
- `ssot-invariants.sh` — Family 6.
- `sibling-coupling.sh` — Family 7.
- `introducing-drift.sh` — Family 8.
- `primitive-promotion-candidates.sh` — Family 9.
- `migration-coverage.sh` — Family 10.
- `consolidation-pair.sh` — Family 11.
- `fixture-session-shape.sh` — Family 12.
- `runtime-hardcoded-paths.sh` — Family 13.
- `installer-registry-parity.sh` — Family 14.
- `runtime-probe-parity.sh` — Family 15.
- `installer-command-parity.sh` — Family 16. `/ductus`'s §Per-Agent Scaffolding slash-command manifest must list exactly the `framework/commands/*.md` files, minus the maintainer-only commands (`audit`) intentionally withheld from adopters.
- `host-namespace-parity.sh` — Family 17. The namespace the runtime *renders* (`[host] project`, else the repo directory basename, per `Host::load`) must match a namespace actually *installed* under an agent config dir (`{cli-config-dir}/commands/<ns>/`, or `command/<ns>/` for opencode). A mismatch means every rendered next-action names a namespace the operator cannot invoke. Not a finding when nothing is installed, or when the basename fallback already agrees — the check asserts agreement between two existing things, not the presence of the `[host]` block.
- `marker-list-parity.sh` — Family 18. The `criterion-path-existence` non-assertion marker list is restated in four places — its canonical source (`specs/045-decision-state-drift-detection/data-model.md`), the runtime array (`NON_ASSERTION_MARKERS`), the adopter-facing copy in `framework/commands/analyze.md` (which cannot be a pointer: adopters have no copy of 045), and the 022 scenario's count. The list decides which findings the family suppresses, so a stale restatement is a canonical source lying about shipped behaviour. Derives from the canonical table and compares the rest as sets, plus the spelled-out counts; an empty derivation is a finding, never a silent pass.
- `review-freshness.sh` — Family 19. No `done` spec ships with a review that predates its own code: flags any whose `scenarios/*.md` or `data-model.md` changed since its `review.reviewed-against`. The release-time half of the rule `check-review-gate` enforces at completion time. Scoped to durable contracts because the two wider rules were measured and rejected — the plan's Affected Files flags 42 of 48 (old specs list shared surfaces every later spec touches), the whole spec directory flags 31 of 48 (`tasks.md` churns on every ticked checkbox). Wired into `run-all.sh` once the 10 pre-existing stale reviews it reported on landing were cleared.
- `version-agreement.sh` — Family 20. The repo-root `version` pin, `runtime/Cargo.toml`, and the newest `runtime/CHANGELOG.md` heading carry the same SemVer. The release tag is deliberately not compared — the release commit precedes the tag push, so asserting it would fail every release mid-flight.
- `transitional-bootstrap-parity.sh` — Family 21. The retired `framework/bootstrap/govern.md` stays byte-identical to `framework/bootstrap/ductus.md`. Every pre-rename adopter's self-update fetch resolves to the retired path, so drift ships them stale content verbatim and a deletion 404s their run before migrations.
- `adopter-shell-behavior.sh` — Family 22. Runs the shipped `ductus-pre-commit` and `.ductus/scripts/**` against an adopter-shaped fixture — non-default `[paths] specs-root`, config only at the converged tier, runtime reachable only through the `.ductus/bin/ductus` pointer. This repo runs different copies of that job, so every assumption they mask is invisible to a green run here. Runtime stubbed, so the family is hermetic; a fixture that cannot be built emits rather than skips.
- `sweep-target-manifest-parity.sh` — Family 23. The live-artifact enumeration a rename sweep greps — delimited in `AGENTS.md` by `<!-- audit:sweep-targets:begin -->` / `<!-- audit:sweep-targets:end -->` — covers every source path the **Shared Files** manifest ships. A list naming a relocated directory sends the grep somewhere clean, which is how 042's move of the generators to `.ductus/scripts/` survived 049's sweep. Runs manifest → list only and says so: it proves no shipped file goes unswept, not that the list is complete. An empty extraction on either side is a finding, never a pass.
- `rename-sweep-residue.sh` — Family 24. The project name never appears where English requires a verb. 049's word-boundary `govern` → `ductus` sweep could not distinguish the project noun from the ordinary verb, and left eight sites reading "the framework should ductus at the rules tier" — three of them shipped to adopters, one into a `create`-strategy template that no later run corrects. Detects on two closed word classes (a modal followed by the name; the name followed by a demonstrative or wh-word), which is exact rather than heuristic: 8 findings before the repair, 0 after. `the`, `to`, and `that` are excluded by measurement, each being ordinary before the name. A scan that examines no files is a finding.
- `unbalanced-inline-markup.sh` — Family 25. `AGENTS.md` and the `AGENTS.md` template carry no line with an odd number of backticks or `**` markers outside fenced code blocks. Spec 050's rewrite of 21 entries left two malformed — an orphan backtick where a bold title was deleted, and a `**` followed by a space that renders literally — and markdownlint sees neither, because an unclosed backtick never becomes a code span. Scoped to two files by measurement, not by omission: their bullets are single-line (69 bullets, 0 continuations) so a per-line check is exact, while the wider corpus wraps bold across lines legitimately on 283 lines. Reports its scope on stderr, and reports a wrapped bullet rather than narrowing once the convention lapses.
- `broken-relative-links.sh` — Family 26. Every relative markdown link resolves. Nothing else covers this: markdownlint's MD051 validates heading *fragments* and never checks that the file exists, and `check-orphaned-references` scopes to adopter-owned referrers and ductus-managed prefixes, so a scenario linking a sibling spec at the wrong depth falls between them. That depth error is the dominant class — a scenario sits one tier deeper than its spec, so a sibling is `../../NNN-other/` and the constitution is `../../../framework/`, and one `../` too few renders fine, reviews fine, and resolves to nothing. 28 such links existed when the family was written. Inline code spans are stripped before matching, which is load-bearing rather than tidy: docs that discuss linking quote link syntax constantly, and without stripping them the family reports 7 false positives, every one a doc correctly describing a link rather than making one. Generated command copies and adopter-facing templates are excluded by construction — their links resolve elsewhere by design — and both exclusions are counted on stderr. A failed file listing is a finding, never a silent pass.
- `done-spec-criteria.sh` — Family 27. No spec at `status: done` carries an unchecked acceptance criterion. The completion gate is supposed to make this unreachable — `implement.md` refuses to propose the transition while any criterion is unchecked — so one that got through means the gate was bypassed or its marking step failed. 026 reached `done` in `e9262df` with AC19 unticked and every signal stayed green: this suite exited 0, `check-artifacts` reported the feature clean, CI passed, and a hand-written grep found it. Only `done` specs are examined, since unchecked criteria are the expected state at every earlier status. Status comes from the frontmatter block and the checkbox from the Acceptance Criteria section, never a loose grep — `status: done` appears in prose across the corpus — and fenced blocks are skipped so a doc quoting checkbox syntax is not reported as carrying one. Each finding names both repairs, tick or reopen, because the family cannot tell which is right. Specs are enumerated from git, so untracked ones are skipped and counted rather than silently ignored; the examined count goes to stderr and an empty enumeration is a finding.
- `audit-family-parity.sh` — Family 28. The family set `run-all.sh` registers, the set `framework/commands/audit.md` enumerates, and the scripts listed in this file all agree. They drifted: `run-all.sh` registered 25 families while `audit.md` stopped at 23, leaving Families 24–26 undocumented — the three added most recently, which is the direction this always runs, since wiring a family into `run-all.sh` is what makes it execute while the doc update has no consequence if skipped. Registered-but-undocumented and documented-but-unregistered are separate findings with separate repairs; the second is worse, since a family dropped from `run-all.sh` whose entry survives reads as still running. Both sets are derived, never hardcoded — a hardcoded expectation would be a third copy of the fact under test — and compared as `(number, script)` pairs so a right-number/wrong-script entry is caught. Retired numbers need no allowance: Family 3 is spent and absent from both, so they agree. An empty derivation on either side is a finding, because two empty sets compare equal.

Families 1–9 are described in detail in the [026 spec](../../specs/026-framework-self-audit/spec.md#check-families) and the [026 plan's Technical Decisions](../../specs/026-framework-self-audit/plan.md#per-family-script-designs); families 10+ were added incrementally and carry their rationale in each script's header comment.
