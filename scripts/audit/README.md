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
- `review-freshness.sh` — Family 19. No `done` spec ships with a review that predates its own code: flags any whose `scenarios/*.md` or `data-model.md` changed since its `review.reviewed-against`. The release-time half of the rule `check-review-gate` enforces at completion time. Scoped to durable contracts because the two wider rules were measured and rejected — the plan's Affected Files flags 42 of 48 (old specs list shared surfaces every later spec touches), the whole spec directory flags 31 of 48 (`tasks.md` churns on every ticked checkbox). **Not currently wired into `run-all.sh`**: it reported 10 pre-existing stale reviews when it landed (a count that drains as reviews are refreshed), so wiring it would block the next release tag until they are cleared. Run it directly, or add the `run_check` line once that debt is gone.

Families 1–9 are described in detail in the [026 spec](../../specs/026-framework-self-audit/spec.md#check-families) and the [026 plan's Technical Decisions](../../specs/026-framework-self-audit/plan.md#per-family-script-designs); families 10+ were added incrementally and carry their rationale in each script's header comment.
