# 085 — The audit-record gate reads records through the runtime Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. `check-audit-records` reads every `done` spec's records (AC2, AC3, AC4)

- [x] `runtime/src/primitives/check_audit_records.rs` walks the corpus as the plan's *The walk is `check-owed-work`'s corpus form* says, reads each `done` spec's records as *Records through the gate's loaders, with a probe for `blocking`* says, reads the baseline as *The runtime reads the baseline file* says, and returns the result [data-model.md](data-model.md) defines, its lists bounded past the read size
- [x] Registered at the seven sites `docs/developing-ductus.md` §Adding or removing a runtime primitive names, its description `CHECK_AUDIT_RECORDS` stating its rules; the CLI prints the report and exits as the data model's *CLI report and exit status* says; `runtime/CHANGELOG.md` records the primitive
- [x] Unit tests hold each finding kind failing with its condition present and passing without it, the baseline's states, an exempt set at and past its mark, an untracked spec, a spec root that cannot be listed, and the empty corpus, from a project at the repository root and from one in a subdirectory (`git_fixture::subdirectory_project`); each behavior has a mutant its test turns red, listed in `scripts/mutation/audit-records-085.py`

- **Done when**: every finding kind and count the data model names is produced and proven by a test its mutant turns red; `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and the whole `cargo test` pass.

## 2. The template's gate step runs the primitive (AC1, AC4, AC5)

Blocked on task 1: the step calls the primitive, and its test runs this tree's release build.

- [ ] `framework/templates/ci/adopter-generators.yml`'s step "Audit-record gate on done specs" runs `ductus check-audit-records`, prints its report, maps each finding line to `::error file=` with `git rev-parse --show-prefix`, and exits with the CLI's status, holding no `${{ }}` expression; the derive step's spec-root extraction and output go; header item 2 says the step runs the primitive
- [ ] `scripts/tests/test-audit-record-gate.sh` runs the step through `scripts/run-workflow-steps.py` from a project at the repository root and from one in a subdirectory, asserts the step ran rather than skipped, and proves each finding fails the step, annotated on its file with the project's prefix, and the clean corpus passes with its counts reported; `.github/workflows/framework-checks.yml` gains a step running it after the runtime build

- **Done when**: the test passes, fails with the step's exit status ignored, and fails with the prefix dropped; `framework-checks.yml`'s and `generators.yml`'s steps pass locally.

## 3. The cost is stated and the affected specs link back (089 AC4, cross-spec impact)

Blocked on task 2: the measured run is the shipped step's.

- [ ] Measure one `check-audit-records` run over this repository's corpus, its wall time by `/usr/bin/time -p` and the bytes it reads, and the description's bytes; state them in `spec.md`'s body with what it retires, as the plan's *What a run costs, and what it retires* says, beside a blockquoted signpost to [089](../089-the-pipeline-s-own-cost-is-stated-and-bounded/spec.md)
- [ ] Blockquoted signposts back-linking 085 in 057's and 066's `spec.md` bodies, saying the step now runs `check-audit-records`, each leaving its spec `done`
- [ ] The prose-claim sweep for the step's old parse (`scripts/stale-literals.sh`, then a search by meaning for the step's inline reader), each stale claim corrected

- **Done when**: `check-review-gate` reports 057 and 066 discharged, the spec states its cost with its method and units, and the sweep leaves no live claim of the old parse.
