# 030 — Cross-Service References Tasks

Tasks derived from the [plan](plan.md). Complete in order. Tests are first-class — each implementation task carries its own verification.

## 1. Registry schema (`[services]`)

- [x] Add the `[services.<alias>]` type (`repo`, `path`, optional `description`) in a new `runtime/src/schema/services.rs` module, parsed from `.ductus/config.toml`; absent table → empty set.
- [x] Confirm `data-model.md` matches the implemented schema (`repo` / `path` / optional `description`). (The §drift-prevention canonical-source row is Task 8.)
- [x] **Done when:** `[services]` parses, a duplicate `repo` is detectable, and a missing table is a no-op; unit tests cover present/absent/duplicate.

## 2. `/{project}:link` registration command

- [x] Create `framework/commands/link.md`: register a service in `[services]` — **prompt for each field one at a time** (alias → repo → path → optional description), validating as entered (unique TOML key, URL-shaped repo, not-checked-out warning on the path); inline positional args remain an optional shortcut; additive write that preserves other `.ductus/config.toml` tables; `--list` shows registered services + resolution health.
- [x] Wire through the command/help-table/permission generators and add per-agent permission entries in `framework/bootstrap/configure/*.md`; register in the command manifest.
- [x] **Done when:** `/{project}:link` adds a well-formed `[services]` block without disturbing other tables, rejects a duplicate alias, warns on an unresolved path, lists services with `--list`, and appears in `/{project}:help`; command tests pass.

## 3. Harvest generator

- [x] Create `scripts/gen-cross-service-refs.sh`: harvest body links whose repo matches a registered `[services]` entry into the `references:` frontmatter field; ignore the branch ref; honor the fenced-block / blockquote / `## See also` exclusions; never touch `dependencies:`.
- [x] Wire it into `scripts/install-hooks.sh` (pre-commit) and `.github/workflows/generators.yml`.
- [x] **Done when:** `--dry-run` is clean on the repo; shell tests cover a matching link (harvested), an unregistered link (recorded with null service), a `## See also` link (excluded), a branch-ref variation (same identity), and confirm `dependencies:` is untouched.

## 4. `resolve-references` primitive + unit tests

- [x] Add `runtime/src/primitives/resolve_references.rs` with `run(args, repo)`; reuse `read_text` / `split_frontmatter` / `ALLOWED_STATUSES`; classify each reference into the closed outcome enum.
- [x] Add Args/Result/outcome types to `runtime/src/schema/primitives.rs`; register in `runtime/src/primitives/mod.rs` and expose in `runtime/src/mcp/server.rs`.
- [x] **Done when:** the crate builds and Rust unit tests (tempdir fixtures with fake registered checkouts) cover all five outcomes — `ok`, `unregistered`, `not-checked-out`, `broken` (missing target + malformed URL), `status-unreadable` (no frontmatter / malformed YAML / out-of-set / scenario target) — plus a self-reference.

## 5. Markdown-only fallback

- [x] Write the runtime-absent procedure into the command prose (Tasks 6–7): read `.ductus/config.toml`, resolve `path`, read the linked frontmatter `status`, classify — using host file tools only, no shell-pipeline substitution.
- [x] **Done when:** the prose path and the primitive produce identical resolution records for the same fixtures; `lint-procedure-parseability.sh` passes.

## 6. `/{project}:status` integration

- [x] Update `framework/commands/status.md` so the status payload includes per-reference resolution records and the dashboard surfaces outcome + linked status, plus the service `description` for orientation when present. The `unregistered` outcome points the user at `/{project}:link`.
- [x] **Done when:** `status.md` documents both paths; the `status-basic` parity fixture/golden are extended with a reference and pass.

## 7. `/{project}:analyze` broken-reference finding

- [x] Update `framework/commands/analyze.md`: a `broken` outcome is an Advisory finding (distinct from the informational unknowns `unregistered` / `not-checked-out`); the `unregistered` surfacing suggests `/{project}:link`.
- [x] **Done when:** `analyze.md` documents the check; the `analyze-basic` parity fixture/golden cover a broken reference and a clean reference and pass.

## 8. Constitution + frontmatter schema

- [x] Add the `references:` row to constitution §text-first-artifacts (generator-managed, derived, distinct from `dependencies:`).
- [x] Add the §spec-lifecycle carve-out: a diff that only adds/removes/changes cross-service reference links is mechanical-class (non-reopening); a `done` spec stays `done`. Word it so the exemption is diff-determinable.
- [x] Add the §drift-prevention canonical-source row pointing at this spec's `data-model.md`.
- [x] Mirror the §spec-lifecycle interaction and the new generator in `AGENTS.md`.
- [x] **Done when:** anchors resolve, the canonical-source/back-link audits pass, and `scripts/audit/run-all.sh` is clean.

## 9. Parity, golden, and fixtures

- [x] Add `runtime/tests/fixtures/cross-service-*` (a consumer spec plus fake registered checkouts exercising each outcome) and `runtime/tests/parity/cross-service/*` with `runtime/tests/golden/cross-service-*.jsonl`.
- [x] **Done when:** markdown-only and runtime paths produce byte-identical golden records; the runtime test suite is green.

## 10. CI opt-in invariant

- [x] Add the `resolve-references` tool name to `framework/runtime-tools.txt` so step (a) verifies it is absent from PATH.
- [x] Add `bash scripts/gen-cross-service-refs.sh --dry-run` to `markdown-only-pipeline.yml` step (b), and ensure the markdown-only job completes status resolution with no runtime present.
- [x] **Done when:** `markdown-only-pipeline.yml` passes with the runtime binary absent — the fallback is exercised end-to-end.

## 11. Documentation

- [x] Update `README.md`: document `[services]` and cross-service reference resolution (local-checkout requirement, outcome semantics), and add the `/{project}:link` row to the **Orient** command table.
- [x] **Done when:** README reflects the shipped behavior, the Orient table includes `/{project}:link`, and `npx markdownlint-cli2` is clean.

## 12. Full validation sweep

- [x] **Done when:** `npx markdownlint-cli2`, the full runtime test suite (`cargo test`), `scripts/audit/run-all.sh`, and every generator `--dry-run` are all green, and `/{project}:review` passes on the implementation.

## 13. Root-aware cross-service harvest (scenario: referenced-service-spec-root)

- [x] Make `scripts/gen-cross-service-refs.sh`'s URL matcher resolve the referenced service's `[paths] specs-root` (read from the registered, checked-out reference's own `.ductus/config.toml`, default `specs`) instead of a hardcoded `/specs/NNN-slug/`; see [scenario](scenarios/referenced-service-spec-root.md).
- [x] Resolve the root-unknowable cases (registered-but-not-checked-out, unregistered) per the scenario's Open Questions, settled in `/ductus:clarify`; reuse 040's `[A-Za-z0-9_-]` regex-safety guarantee for the interpolated root.
- [x] Keep the markdown-only fallback and runtime harvest paths producing a byte-identical `references:` index (030 parity invariant).

- **Done when**: shell tests cover a referenced service with a renamed root (harvested), a referenced service on the default `specs/` root (unchanged), and a registered-but-not-checked-out reference (per the resolved open question); `gen-cross-service-refs.sh --dry-run` stays clean on this repo and the cross-service parity goldens still match.

## 14. Review and analyze the corrections spec 079 made

Spec 079 task 10 corrected `data-model.md`, scenario `referenced-service-spec-root`, AC10 and the plan, which stated a markdown-only path; a scenario or data-model edit stales the review. It owes `/ductus:review`, then `/ductus:analyze`, then the completion gate, which run once no box in this file is open (spec 079).

The review is recorded at `8339e0f6`. It fixed the broken contract it found as the owner chose (2026-11-07): `data-model.md` and `/link` called a repo under two aliases a registry-validation finding while `Services::duplicate_repos` had no caller, and `resolve-references` now returns `duplicate-repos`, which `/analyze` step 13 reports. Rejected then: the contract made to match the code, with the dead helper removed, which leaves a hand-edited duplicate unreported; and a deferral to a new task here. The analysis that followed deferred three findings to task 15.

- **Done when**: a review and an analysis are recorded on this spec, and `/ductus:implement` has returned it to `done`.

## 15. The rule findings 030's analysis deferred here

The analysis of 2026-11-08 found these against rule text that gained the coverage after this spec's last analysis (CFG-ENV-003's file keys in `6b0f52da`, CFG-ENV-008's no-fallback clause in `1598b87a`), and the owner chose (2026-11-08) to fix them in this reopening. Rejected: deferring to 022, which releases this spec but moves a feature-specific primitive's contract out of its feature; and closing other specs first, which keeps a known miss out of 079's AC20 sample. The code lands after the first review, so this spec then owes a second `/ductus:review`, then `/ductus:analyze` and the completion gate, and its reopening closes over AC1's bound, a miss spec 079 records with its cause.

- [ ] CFG-ENV-003: a `[services.<alias>]` entry whose `repo` or `path` is absent or empty halts every `[services]` reader, naming the alias, the field and the file, as a `[constitutions.*]` entry does (`runtime/src/primitives/resolve_constitutions.rs`); an empty `path` now resolves to the consumer repo and an empty `repo` is skipped, and the data model commits to the check
- [ ] CFG-ENV-008: a config that does not parse halts `derive-references` naming the file, where it now reads as an empty registry and `--write` gives every `references:` entry a null `service`; the plan states each reader's parse failure
- [ ] BE-PAYLOAD-001: `resolve-references`' result is bounded by `[artifacts] read-size-bytes`, past which its two lists move whole to the file `lists-file` names, as 066's scenario `scope-results-have-a-bounded-size` has `compute-review-scope` do; the data model states the bound
- [ ] Each behavior has a test that fails without it and a mutant in `scripts/mutation/cross-service-references.py`, and the descriptions, `runtime/CHANGELOG.md` and `docs/cross-service-references.md` follow

- **Done when**: each finding's behavior is in the code with a test shown to fail without it and a killed mutant, and the data model and plan state each commitment
