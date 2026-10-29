# 036 — Cross-cutting code-quality rules Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Author `framework/rules/quality-cross.md`

- [x] Write the file header: title (`# Code Quality Rules`), an intro stating the discipline is cross-cutting (applies to every stack), the RFC 2119 note, the `QUAL-{CATEGORY}-{NNN}` ID-format / category-declaration line (category `STUB`, with the `See specs/036-quality-cross-rules/data-model.md` + `specs/008-security-rules/data-model.md` pointers), and the pin note adapted for a cross file (always applies; pin in `.ductus/config.toml` `[pinned]` if customized).
- [x] Write the `## QUAL-STUB — Silent stubs` section with `### QUAL-STUB-001`: Statement (MUST), Rationale (silent-stub hazard + the adopter rate-limiter incident), and Verification (review-time three-part discriminator — reachable + contract-implies-work + no-loud-signal — plus the exemption list).
- [x] Cross-reference rather than restate: cite `api-backend.md` `BE-SCHEMA-002` for the build-time schema fail-loud case.
- Done when: the file exists with one well-formed `QUAL-STUB-001` rule (MUST), the `-cross.md` schema, and the `BE-SCHEMA-002` citation.

## 2. Register the `QUAL` surface

- [x] Write `specs/036-quality-cross-rules/data-model.md` registering the `QUAL` surface and `STUB` category, referencing 008's schema (already drafted at plan time — confirm it matches the shipped rule file).
- [x] In `scripts/lint-rule-ids.sh`, extend the allowlist regex `^(BE|FE|CFG)-…` to include `QUAL`, update the error-message string to `{BE|FE|CFG|QUAL}`, and add `specs/036-quality-cross-rules/data-model.md` to the "Source of truth" comment block.
- Done when: `scripts/lint-rule-ids.sh` accepts `QUAL-STUB-001` and still rejects malformed IDs; the data-model and the rule file agree.

## 3. Register the file in the `/ductus` manifest

- [x] Add `framework/rules/quality-cross.md → specs/rules/quality-cross.md` to the `### ductus-owned shared files` table in `framework/bootstrap/ductus.md`, slotted between `performance-frontend.md` and `security-backend.md` (strategy: update).
- [x] Make the §Shared Files "Rule-file surface filter" note count-free (dropped the hard-coded number, which had silently drifted — it read "six" while seven rule files were already listed — and which nothing machine-checks; removing it eliminates the drift class rather than just correcting the value).
- Done when: the manifest row is present, the note no longer hard-codes a rule-file count, and the `-cross.md` suffix makes 024's loader select it for every stack and 033's filter keep it unconditionally.

## 4. Validate

- [x] `scripts/lint-rule-ids.sh` passes (`QUAL-STUB-001` well-formed and accepted; categories disjoint from `BE`/`FE`/`CFG`; no duplicates).
- [x] `scripts/lint-rule-filenames.sh` passes (the `-cross.md` suffix).
- [x] `npx markdownlint-cli2`, the other `scripts/lint-*.sh`, and `scripts/audit/*` pass (frontmatter, tool-coverage, procedure-parseability, manifest-parity, ssot-invariants, cross-doc-consistency — all green).
- [x] `STUB` confirmed disjoint from existing categories; `QUAL` confirmed disjoint from `BE`/`FE`/`CFG` surfaces.
- Done when: all lints/audits green.

## 5. Review and complete

- [x] Run `/ductus:review` over the change set; resolve any MUST findings.
- Done when: `/ductus:review` reports no blocking violations and the spec can advance to `done`.

## 6. Add QUAL-GROUND-001 (verify external contracts)

- [x] Declare the `GROUND` category in the `quality-cross.md` file header and register it in `data-model.md` (category table + `QUAL-GROUND` namespace).
- [x] Add `QUAL-GROUND-001` (SHOULD) to `quality-cross.md` following the canonical schema — the code-side counterpart to `/ductus:analyze`'s grounding check, enforcing constitution §grounding.
- Done when: `scripts/lint-rule-ids.sh` accepts `QUAL-GROUND-001`, markdownlint and `scripts/audit/*` pass, and the rule is registered in both the file header and the data-model. Re-run `/ductus:review` before advancing 036 back to `done`.

## 7. Add QUAL-CLAIM-001 (unsubstantiated clean results)

- [x] Declare the `CLAIM` category in the `quality-cross.md` file header and register it in `data-model.md` (category table + `QUAL-CLAIM` namespace).
- [x] Add `QUAL-CLAIM-001` (SHOULD) to `quality-cross.md` following the canonical schema, with the promotion criterion to MUST documented alongside the Verification paragraph.
- [x] Cite the four originating instances in the rule's Source paragraph so the rule is traceable to observed defects rather than speculation.

- **Done when**: `scripts/lint-rule-ids.sh` accepts `QUAL-CLAIM-001`; the category appears in both the `quality-cross.md` header and `data-model.md` (table + namespace section); markdownlint and `bash scripts/audit/run-all.sh` pass; `/ductus:review` is re-run before 036 advances back to `done`.

## 8. Add QUAL-DELEG-001 (delegation to shared code)

- [x] Declare the `DELEG` category in the `quality-cross.md` file header, per the per-file category-declaration policy
- [x] Register the category in `data-model.md` §Category abbreviations and add the `QUAL-DELEG` namespace entry under §Rule set
- [x] Author the rule as SHOULD, with a Verification clause naming the enumeration a delegation owes and treating a test exercising both the widened and the narrowed direction as compliant
- [x] State the discriminator against `QUAL-CLAIM-001` and `QUAL-GROUND-001` in the Rationale, as `QUAL-CLAIM-001` does against its siblings
- [x] Carry the `050-constitution` signpost in the spec body so the declared cross-spec impact is discharged by a reciprocal link rather than by deleting the key
- [x] Verify: `scripts/lint-rule-ids.sh` passes, `check-rule-ids` reports the ID known against `examined: 11`, and `npx markdownlint-cli2` is clean

- **Done when**: `framework/rules/quality-cross.md` carries `QUAL-DELEG-001` with Statement / Rationale / Verification, the `DELEG` category is declared in the file header and registered in `data-model.md`, `scripts/lint-rule-ids.sh` and `check-rule-ids` accept the ID, and `050-constitution`'s `cross-spec-impact:` declaration is discharged by the signpost this spec carries.

## 9. Move QUAL-DELEG-001's discriminator into its Rationale

- [x] Move the sentence distinguishing `QUAL-DELEG-001` from `QUAL-CLAIM-001` and `QUAL-GROUND-001` out of the Verification paragraph and into the Rationale of `framework/rules/quality-cross.md`, where both siblings state theirs
- [x] Verify: `scripts/lint-rule-ids.sh` passes, `npx markdownlint-cli2` is clean, and the full `cargo test --release --locked` passes, since `framework/rules/**` is a runtime CI input

- **Done when**: `QUAL-DELEG-001`'s Rationale states its discriminator against `QUAL-CLAIM-001` and `QUAL-GROUND-001`, its Verification no longer carries it, and the rule's ID, Statement and trigger are unchanged — so AC10 holds as written.

## 10. QUAL-TEST-001 — a test fails when the behavior it names is removed (routed by 050's third round)

- [x] `framework/rules/quality-cross.md`: add `QUAL-TEST-001` (SHOULD) with Statement, Rationale (its discriminator against `QUAL-CLAIM-001` and `QUAL-STUB-001`), Verification and Source, and declare the `TEST` category in the header
- [x] `data-model.md`: register `TEST` under Category abbreviations and add the `QUAL-TEST` namespace; extend the added-rules sentence, which enumerates the shipped set by design and says to extend it whenever a rule is added
- [x] `spec.md`: a `TEST` bullet under Added categories, the 050 signpost extended to name the third round, and an acceptance criterion for the rule
- [x] `framework/commands/analyze.md`: its example of the categories `quality-cross.md` declares is stale (it omits `DELEG`); restate it without enumerating the set
- [x] Verify: `scripts/lint-rule-ids.sh`, `npx markdownlint-cli2`, the audit, and the full `cargo test --release --locked`

- **Done when**: `framework/rules/quality-cross.md` carries `QUAL-TEST-001` with all four fields, the `TEST` category is declared in the header and registered in the data model, 050 is linked back, and the local gate passes

## 11. QUAL-GROUND-001 and QUAL-DELEG-001 say a SHOULD finding does not block done, which §implement-phase contradicts

- [x] Both Verifications end "it does not block `done`" / "does not block `done`", but constitution §implement-phase holds that a spec does not reach `done` with an outstanding SHOULD, and analyze's review-state drift reports a `done` spec whose review records one (`runtime/src/primitives/check_artifacts.rs:665`). Restate both as QUAL-TEST-001 does: the finding never sets the review's `blocking` flag, and like any outstanding SHOULD it is fixed or waived before the spec reaches `done`. Surfaced writing task 10

- **Done when**: no rule in `quality-cross.md` claims a SHOULD finding leaves `done` unblocked

## 12. Rule-file review 2026-10-19: land the QUAL-TEST-001 trigger amendment, the QUAL-CLAIM-001 fixes and the header opt-out rewrite

- [x] `framework/rules/quality-cross.md` header: the opt-out paragraph says cross files load for every stack (`[rules] surfaces` cannot exclude them; `/{project}:analyze` loads every rule file present), names `[[review.disabled-rule-files]]` with its mandatory `reason` as the review exclusion, and says `[pinned] files` only stops updates overwriting a customized copy
- [x] `QUAL-CLAIM-001` Verification: it never sets the review's `blocking` flag, and like any outstanding SHOULD it is fixed or waived before the spec reaches `done`
- [x] `QUAL-CLAIM-001` Source: `run-all.sh` in past tense, and the derive primitives' field list names `examined`, `untracked-skipped`, `unwritten`, `absent` and `unparseable`, matching `derive_dependencies.rs` / `derive_references.rs`
- [x] `QUAL-TEST-001` Verification: trigger (4), a test whose ability to fail depends on a condition it does not establish, and trigger (5), a test that never reaches the unit it names, with the matching compliant cases (establishes the condition, gated as a whole, doubles collaborators but calls the unit)
- [x] `QUAL-TEST-001` Source: the 062 review finding (`cb54dc8f`, 2026-09-29) and the 048 review findings (`f7c14890`, 2026-09-30) the new triggers came from
- [x] `data-model.md` QUAL-TEST namespace names the new triggers and compliant cases
- [x] `spec.md`: the `TEST` bullet under Added categories names the new shapes, and AC12 covers triggers (4) and (5)
- [x] `plan.md`: the header-note decision states the opt-out that exists (`[[review.disabled-rule-files]]`) and what `[pinned]` does not do
- [x] Verify: `npx markdownlint-cli2` on the changed spec files and `scripts/lint-frontmatter.sh` pass

- **Done when**: 036's spec, plan and data model describe `quality-cross.md` as it stands after the 2026-10-19 rule-file review, with no rule ID, Statement or category changed. Re-run `/ductus:review` before 036 returns to `done`, since the data-model edit stales its completed review.

## 13. Rule-file review 2026-10-19 (second pass): QUAL-GROUND-001 covers in-repo contracts and cedes configuration to CFG; source and wording fixes

- [x] `framework/rules/quality-cross.md` `QUAL-GROUND-001` Statement, Rationale and Verification: the rule covers code depending on a contract defined outside that code, including a list, schema, or format defined elsewhere in the repository; the in-repo exemption is replaced by a contract used through its own definition, so the compiler or type checker enforces it; a schema or migration reference counts only when the build or a test checks it; configuration keys and values are left to `configuration-cross.md` `CFG-ENV-003` / `CFG-ENV-008`
- [x] `quality-cross.md` header gloss reads `GROUND` (unverified contracts) and the section heading reads `## QUAL-GROUND — Verify contracts`
- [x] `quality-cross.md` `QUAL-STUB-001` Verification cites `BE-SCHEMA-002` for the API-schema fail-loud case (a published schema bound to its implementation), no longer for a build-time case
- [x] `quality-cross.md` `QUAL-STUB-001` and `QUAL-GROUND-001` Sources cite Jim Shore, "Fail Fast", IEEE Software, September/October 2004, pp. 21–25; checked against martinfowler.com/ieeeSoftware/failFast.pdf, whose byline, issue line and page folios 21–25 match and whose worked example returns a default for a missing configuration property where the fail-fast version throws. Verify-only: no 036 artifact restates a Source
- [x] `quality-cross.md` `QUAL-DELEG-001` Source records the 2026-09-14 link-slug delegation incident. Verify-only: no 036 artifact restates a Source
- [x] `spec.md`: the Motivation's `BE-SCHEMA-002` sentence and the first Boundaries bullet describe the API-schema case rather than build-time; a second Boundaries bullet names `CFG-ENV-003` / `CFG-ENV-008` as the owners of configuration; the `GROUND` bullet under Added categories matches the new Statement
- [x] `spec.md`: AC6 names the API-schema case, and AC9 says the Source records the outcome of assessing each candidate that shared only the message shape, since no instance is unassessed any more
- [x] `spec.md`: a Resolved Question, `QUAL-GROUND-001` scope, records the 2026-10-19 owner decision and its reasons (the exemption contradicted the Statement's own examples; 026's and 022's reviews applied the rule to in-repo contracts; a configuration defect would surface under two IDs, which `write-review`'s `(rule, file)` dedup does not collapse)
- [x] `data-model.md`: the category row reads Unverified contracts, the namespace heading reads (Verify contracts category), the `QUAL-GROUND-001` summary matches the new Statement and the `CFG` cession, and the `QUAL-STUB-001` summary names the API-schema case
- [x] `plan.md`: the `QUAL-STUB-001` Verification bullet cites `BE-SCHEMA-002` for the API-schema case, and Cross-spec impact lists `CFG-ENV-003` / `CFG-ENV-008` as cited, not changed
- [x] Verify: `npx markdownlint-cli2` on the changed spec files and `scripts/lint-frontmatter.sh` pass

- **Done when**: 036's spec, plan and data model describe `quality-cross.md` as it stands after the second 2026-10-19 rule-file review: `QUAL-GROUND-001` covers in-repo contracts and cedes configuration to `CFG`, `BE-SCHEMA-002` is cited for the API-schema case, and no criterion claims an unassessed instance. The rule IDs and tiers are unchanged. Re-run `/ductus:review` before 036 returns to `done`, since the data-model edit stales its completed review.

## 14. Implement scenario: a-bound-test-pins-the-stated-figure

- [ ] Implement the behavior described in `scenarios/a-bound-test-pins-the-stated-figure.md`

- **Done when**: The scenario's open question is resolved and quality-cross.md states the requirement in the chosen form, with check-rule-ids accepting the file.
