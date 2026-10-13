---
spec: 026-framework-self-audit
scenario: family-39-shipped-anchor-resolution
last-run: 2026-10-13T15:58:06Z
reviewed-against: 1bc3ad6446b18b75dd363e397cbaab9126649d9e
diff-base: bd14fefe7d05788877b7cde6ed7a850be1b7fb74
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 7
scope: 21
skipped-passes: []
reviewed-digest:
  scenarios/audit-ci-hard-gate.md: a7bad7a167532019112d79a746696ad32d963171598d2283072af0e9f3234be7
  scenarios/audit-script-refactors.md: 035bb7ef52c236135d791f35c3ec407d7908c8d51a169483bce40bc26c7a46c8
  scenarios/family-10-migration-coverage.md: 8684e643e4ea938cbeeed6d1342aef27efc84a7bf7559b111b6a12cacacfa02b
  scenarios/family-17-contract-binding.md: 1ab3dfbc5c53ca5ab082b587a299b1951c8dd3c3e29148db5ed3a5df50903faf
  scenarios/family-18-marker-list-parity.md: 8d48555ad2ec2a7983bfa8dac24c6edf9dcbc051037ae6b00c9ec5e26f767e4d
  scenarios/family-19-mechanical-sweep-exemption.md: 1aed9678cb90da55fb314f9f8bd26addb3baebdbad04bda48fb11deeb95c4089
  scenarios/family-19-review-freshness.md: 07adf4f5f7f0590e90c043d9254534f10f77fc717fb951bd7dfe2ff42a74d787
  scenarios/family-19-says-what-it-examined.md: 2ee343d1295dfc755ea19ae453e5edd61dfb7cdb3d1eecdd46865d54be48621d
  scenarios/family-22-adopter-shell-behavior.md: 2eac3b7db9587dd354168941be8a1816d11a9e6f7fb27ecd8b8f9c9f8f719754
  scenarios/family-23-sweep-target-manifest-parity.md: 9dfa1299cdeb27ad691c66d6e9adeb68187c7c60039576cafd6ba55f29274b38
  scenarios/family-24-rename-sweep-residue.md: 41d6c829f0851a3caab11650d24b7100992667969ee3f21454141b482819e132
  scenarios/family-25-unbalanced-inline-markup.md: bddc8f9f37f21e3404f24bf77f732d2adc591faa76038983b3b44a41e640e1c6
  scenarios/family-26-broken-relative-links.md: 60627b24354610f3e0f819abe7d0603faa9644c27e8cee5163a452804bb94b6c
  scenarios/family-27-done-spec-unchecked-criteria.md: d164ba1fedaababcb88f5a4062b150ae8bd7e017699d7ac6d310292a25489bf9
  scenarios/family-28-audit-family-registry-parity.md: 0d7b15a3c103b50b3aeafa3145941910582c16dbda08148ebd804346da4ae15a
  scenarios/family-34-step-reference-integrity.md: 4d24ed6a08c54ae98f135ad108d3b8a7b85f782f5af7edf26c67d92aa509f067
  scenarios/family-35-manifest-destination-links.md: 8e94dce4172e2b326612da806327af31777d99b8c6ce89642c9d10a98dc148b3
  scenarios/family-36-self-url-resolution.md: 23716b0f5c3810890bab089c65258008842869e35616a71b86b663ed231bde18
  scenarios/family-39-shipped-anchor-resolution.md: 989bc2bab609358c00183fd3711d0530f0ebb29a51e4c92f92d37192bce9b355
  scenarios/host-namespace-parity.md: 05714fe6b728391f699ed7328e1aea252a48259ae489fa9c06e0d0609dc1d376
  scenarios/link-check-consolidation.md: f838133a535aa090e09d7f82903facc4dff8cc822c1dec5c22f9b4d1e17a5049
  scenarios/readme-command-parity.md: 3788aa1103dba1860af8cb9950a6425ed33e4a24498f825fcd980e0c9bb7f8bc
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 026-framework-self-audit

## Summary

Five passes over bd14fefe..1bc3ad64: scenario `family-39-shipped-anchor-resolution` (task 42, AC26), the new `scripts/audit/shipped-anchor-resolution.sh`, and its registration in `run-all.sh`, `framework/commands/audit.md` and `scripts/audit/README.md`. Rule files: the eleven `discover-rule-files` selects; `quality-cross.md` governs a check script and was applied in full. QUAL-CLAIM-001 is met by the stderr coverage line (files examined, references by kind, qualified named as excluded by construction) and by the three cannot-examine states each being a finding. QUAL-TEST-001 is met by mutation: reverting `ductus.md:901` to a short heading turned the family red at that line, and an empty enumeration, no reachable runtime and a runtime exiting non-zero each produced a finding and exit 1 in throwaway trees, beside a control that exited 0. The security pass found a list-form subprocess over git-enumerated paths and no new input source; reuse, efficiency and simplicity found nothing (it reuses `ductus_bin` and the README's documented python convention, and makes 64 short subprocess calls). A prose-claim sweep for stale family counts found none. No MUST or SHOULD finding and no observation. **Examined 7 of 21.** The 7 are the window's modified files: the new script and the scenario read in full, the other five read as their diffs over the diff base. Two modified files were not read and are not counted: the generated `.claude/commands/ductus/audit.md` and `.pi/prompts/ductus-audit.md`, which `scripts/gen-claude-commands.sh` reports in sync with their source. The other 12 scope entries are plan-affected paths the window does not touch (the other family scripts, two workflows, `legacy-prose-commands.txt`); they were not read.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
