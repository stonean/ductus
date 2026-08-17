---
spec: 048-govern-acquired-runtime
reviewed-at: 2026-08-17T22:13:28Z
reviewed-against: 0036c9ab96aedfe4143b5431a0c37c5c48a9d6a3
diff-base: b9da1c12396b794c6c160fcc22ae2e6272e59b9c
must-violations: 0
should-violations: 0
low-confidence: 1
captured-issues: 2
skipped-passes: []
---

# Review — 048-govern-acquired-runtime

## Summary

Re-review after the previous verdict went stale on this spec's own scenario, which was corrected to name the right cutoff for the inherent first restart. **0 MUST, 0 SHOULD outstanding, 1 low-confidence — not blocking.** Nominal scope is 553 files against a diff base predating the 049 rename, but the delta since the last review is small and mostly already reviewed: four prose fixes across `framework/bootstrap/ductus.md`, its byte-identical mirror, and `framework/migrations/ductus-rename.md`; the `check-orphaned-references` change reviewed in full under 022 (both scopes contain it); the release-gate fix adding `sbom` to `publish`'s `needs`; and the `[migrations].applied` revert, which nets to zero against the bootstrap. What this review is actually worth is that the fixes were verified against the thing they were written for rather than against tests alone. Two real adopter bootstraps ran on 2026-08-17, both on the strongest available subject — a pre-rename adopter with a legacy root config, a root `constitution.md`, a `workflows/` directory, and an `.mcp.json` naming the retired server key and the bare command. Both cost two restarts, the inherent self-update hop plus the closing hand-over, which is what AC10 states as reworded. Run 1 produced six defects, none reachable by any check here because each needs a pre-rename adopter; run 2 validated the fixes for them one by one. The abort now names the command the adopter can actually invoke, which is why the operator re-ran the retired entry point without having to deduce it. The self-update overwrote the installed file rather than the canonical name, so the tree ended with one command file renamed in git rather than a fresh copy beside a stale one that the rename migration would then have clobbered. `ductus-rename` step 9 removed the retired MCP server and its permission entry by contract rather than by host judgment — run 1 had resolved that collision correctly by luck, which the fix converts into a rule. And the decisive one: `check-orphaned-references` reported `AGENTS.md:118 names scripts/gen-spec-deps.sh, which moved to .ductus/scripts/gen-spec-deps.sh` — the precise orphan run 1 reported clean over and a human found by reading the file. AC13 was validated incidentally, the pointer having gone missing across the reset and being repaired by the run, which is exactly what that criterion promises. One of the six was not a framework defect at all but a misreading of mine, and the correction is recorded in the spec body rather than here: a migration absent from run 1's summary was read as a silent skip when `workflows-sunset` carries `sunset_after = "0.25.0"` against a `0.29.9` release and was correctly excluded as retired. A `[migrations].applied` state model was built on that and reverted once the premise was checked, so both runs turn out to have behaved correctly throughout. The one entry standing is carried unchanged from the previous review: `CFG-ENV-001` in `fetch_archive.rs`, in scope because acquisition is this spec's subject, recorded low-confidence for the reasons given there. Both captured issues are genuinely open and neither belongs to this spec's work — a deferred architectural exploration on hold since 2026-07-11, and a one-word repair in a done spec's body held pending a constitution ruling. Whole-surface evidence at this HEAD: 956 lib tests plus 13 suites green; `clippy --release --all-targets -D warnings` exit 0; `fmt --check` clean; markdownlint clean; `scripts/audit/run-all.sh` exit 0 run after committing; and `ductus-v0.29.9` green across all five platforms with its acquisition legs verified against the published assets. **AC10 stays unticked** — it is the operator's to close, and this review does not close it.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: CFG-ENV-001 — insecure-host allowlist is read from the environment per call rather than cached once

- **File**: `runtime/src/primitives/fetch_archive.rs:295-299`
- **Rule**: All environment variables MUST be read once at startup and the value cached; per-call reads from os.environ (or equivalent) are forbidden.
- **Finding**: Carried unchanged from the previous review; the file is in scope because acquisition drives `fetch-archive` and is this spec's subject. `host_is_insecure_allowed` reads the variable on each call, and `validate_fetch_url` calls it for the initial URL plus every redirect hop, so one fetch performs up to 1 + MAX_FETCH_REDIRECTS reads rather than one cached read. Low-confidence because none of the rule's stated harms land here: the count is bounded rather than a hot path, nothing in the crate mutates the environment in-process so a cached value would be identical, and the default is documented at the constant. Re-reading per hop is defensible as part of screening each redirect target independently.
- **Auto-fixable**: yes
- **Suggested fix**: Wrap the parsed allowlist in a `std::sync::LazyLock<Option<Vec<String>>>`, or resolve it once in `run` and thread it into `validate_fetch_url`. Safe as written, since nothing mutates the environment in-process and the parity tests set the variable on a subprocess before it starts.

## Waived findings

*None.*

## Captured issues

- Architectural exploration: re-frame the runtime's LLM extension points as named Anthropic-style Skills the host loads at the seam, rather than ad-hoc JSON envelopes. Speculative, depends on the Skills protocol stabilizing, and a larger redesign than 022's current scope. **On hold per user 2026-07-11** and still outstanding — not this spec's work.
- [ ] chore: `specs/045-decision-state-drift-detection/spec.md:133` carries a one-word sweep artifact ("a the derive-don't-ask principle"). Still outstanding, and deliberately so: repairing it would reopen a `done` spec to fix a word that changes no claim, and whether that counts as a mechanical edit is a constitution question logged for `050-constitution`. The matching instance in that spec's `plan.md` was repaired in `4070589`. Not this spec's work.

## Observations

*None.*

## Skipped passes

*None.*
