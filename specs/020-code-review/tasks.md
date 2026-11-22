---
status: draft
next-task: 23
---

# 020 — `/ductus:review` Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 18. AC17: a review loads a rule file only when its header names review

- [x] Update `plan.md`'s design record for AC17 first: the header test (the paragraph after the file's H1 names `/{project}:review`), where it runs (`discover-rule-files`' review selection, beside `[[review.disabled-rule-files]]`; `review_pass_for_rule_file`), and the notice naming each file left to analysis. At `d5fc51ec` seven shipped files name `/{project}:analyze` alone: `api-backend.md`, `concurrency-backend.md`, `observability-backend.md`, `performance-backend.md`, `reliability-backend.md`, `accessibility-frontend.md` and `performance-frontend.md`; `security-*.md`, `quality-cross.md` and `configuration-cross.md` keep their passes. The decision is the scenario `every-loaded-rule-file-has-a-pass`'s Resolved Question, marked Superseded in part (2026-11-21).
- [x] Implement with a fixture rule file of each kind, proven to fail with the header test removed.
- [x] Update `framework/reference/review.md` §Load rules and step 2 of `framework/commands/review.md`; read 022's data model §`performReview` and 024's loader for statements AC17 makes false, and add each spec it changes to `cross-spec-impact:`.

- **Done when**: A review of a project loading an analyze-only rule file leaves it out with the notice, the fixture proves both kinds, and the reference and command say so.

## 19. AC18: a finding is recorded only when its trigger can occur in the project

- [x] Update `plan.md`'s design record for AC18. An adopter's review on 0.60.0 spent about four hours of fix rounds on cases a scratch directory could build but the project could not reach (a sibling directory named `{name}.run`, a nested project named `claim`): `framework/reference/review.md` §Run review passes checked a finding's mechanism against the tree, never whether its trigger could arise. The rule entered that section with 050's task 42 (`0f716c7b`), so add 050 to `cross-spec-impact:` if its spec states the rule.
- [x] Reword §Run review passes and §Passes in a subagent so a finding, observation or claim carries the evidence that its trigger is possible in the project (its code, configuration, history or deployment shape), and a case shown only by building it is recorded nowhere. 078 carries the same bar into the re-read brief (its AC16); keep the wording of the two consistent.

- **Done when**: The review reference states the trigger bar for passes and claims, and a review's criteria step can judge AC18 against it.

## 20. AC19: a stored disposition covers only the findings at its (rule, file, site) anchor

- [x] Update `plan.md`'s design record for AC19 first, then `data-model.md`'s Waiver record, Expiry rule, Finding record and `decisions` row to match: the site a pass names (`(top level)` outside any named unit), the anchor triple, the `covers` count (lowered, never raised), an anchor recorded without a site covering nothing, and the record shapes — a waiver entry's anchors with their sites and counts, beside `waiver-file-lists`' one-path-or-list rendering, and a violation deferral's key and fields. An adopter reported on 0.62.0 that a waiver whose reason covered one failure path in a large shared file also absorbed unrelated catch sites of the same rule there; `waiver_reason` and `violation_key` (`runtime/src/primitives/write_review.rs`) match on rule and file alone, and `FiredFinding` carries no more. The three decisions are the scenario's Resolved Questions (2026-11-22).
- [x] Implement: a `site` on `ReviewFinding` and `FiredFinding`; `process-waivers`, `process-decisions` and `write-review` matching on the triple with the count guard and the three `waiver expired:` notices; `--waive` and `--defer` anchoring at each site with its count and refusing a finding that names no site; the `performReview` finding shape and the exec golden fixtures. A fixture of each AC19 case, proven to fail with its guard removed.
- [x] Update `framework/reference/review.md` §Run review passes (the finding's site), §Waivers, §Per-run waiver processing, §Malformed and duplicate waivers and §Deferrals, `framework/commands/review.md` steps 8, 9 and 11 and its `--waive` and `--defer` rows, `docs/rules.md`'s waiver paragraph, and the 076 signpost in this spec's §Frontmatter schema; run the prose-claim sweep for `(rule, file)` and `rule {rule-id} at {file}`.
- [x] Discharge `cross-spec-impact:`: 022's scenario `review-runtime-acceleration` (its code-movement claim), 058's data model (the violation key form), 066's data model (`process-waivers`) and scenario `process-waivers-file-lists`, and 076's data model (the `key` and `decision-key` rows) each carry a blockquote signpost back-linking this spec with the contract change, and `check-review-gate` reports the impact discharged.

- **Done when**: A review judges AC19 met; each case of the scenario `waiver-expiry` has a fixture that fails with its guard removed; the reference, the command and `docs/rules.md` state the triple anchor; and 022, 058, 066 and 076 carry the signposts.

## 21. AC19: findings at different sites are counted apart

- [ ] Update `plan.md`'s AC19 decision first: an anchor's count collapses overlapping findings of one rule, file and site, never across sites. `collapse_overlapping` (`runtime/src/primitives/process_waivers.rs`) collapses a rule's overlapping findings in a file whatever their sites, and a whole-file finding (an empty `line-range`) overlaps every range, so when one is listed in `fired` before a finding at a named site, the waiver anchored at that site counts none and expires with the `no longer fires` notice while its finding stands. `write-review`'s cross-site dedup (AC13) stays as it is, and `lower_covers` already never lowers `covers` to zero, so a site anchor whose finding the dedup kept elsewhere is applied and unchanged. Found reviewing task 20's runtime work, 2026-11-22.
- [ ] Implement with a test, proven to fail with the site condition removed from the collapse, in which a whole-file finding listed before a site finding of the same rule leaves the waiver anchored at that site applied.

- **Done when**: A waiver anchored at a site applies while a finding of its rule fires there, whatever overlapping finding of that rule at another site `fired` lists first, the test fails with the site condition removed, and the plan states the per-site count.

## 22. AC19: only a violation's key reads as recorded before sites

- [ ] Update `plan.md`'s AC19 decision first: a stored key reads as a violation's recorded before sites only when the token after its leading word `rule` is a rule ID. `decisions::is_pre_site_violation_key` (`runtime/src/primitives/decisions.rs`) reads any key that starts with the word `rule`, holds the word `at` and no backtick that way, so an observation's key with no path, such as `rule engine at startup leaks`, expires its stored decision on every review with the `recorded before sites` notice and the person is asked again. Test the token with `check_rule_ids::is_a_rule_id` (`runtime/src/primitives/check_rule_ids.rs`), never a copy of its pattern (`QUAL-DELEG-001`). Found reviewing task 20's runtime work, 2026-11-22.
- [ ] Implement with a test, proven to fail with the rule-ID test removed, in which a waived observation keyed `rule engine at startup leaks` matches while it fires and a deferral keyed `rule BE-X-001 at src/a.rs` still expires as recorded before sites.

- **Done when**: Only a key whose token after its leading word `rule` is a rule ID reads as recorded before sites, the test fails with that check removed, and the plan says so.
