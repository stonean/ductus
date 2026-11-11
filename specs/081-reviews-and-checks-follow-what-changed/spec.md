---
status: draft
dependencies: [079-review-and-analysis-work-converges]
cross-spec-impact: [026-framework-self-audit, 050-constitution, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges]
next-criterion: 8
---

# 081 — Reviews and checks follow what changed

`/{project}:review` is responsible for code and `/{project}:analyze` for the project's artifacts. A reopening whose own work changed only artifacts owes an analysis and no review. A change confined to artifacts owes the artifact checks, and no build, automated test, code lint or code CI.

## Motivation

Specs reached `done` slowly because reviews and checks were owed for changes outside what the work in hand had touched.

- **Reviews owed for no code.** Measured at `6158d7df` on 2026-11-11 with `compute-review-scope` and `check-review-gate`, run from a release build of that tree: all 12 `in-progress` specs waiting only on the completion gate were blocked on a review. Ten of them (002, 010, 014, 023, 024, 036, 038, 045, 051, 058) had no commit of their own touching a path outside `specs/`. Here a commit is the spec's own when its subject's scope names the spec first. The other two, 026 and 029, had one each (`e08ea5b7`, `6cd79d85`). None of the 12 review records held a gating count or an open finding, and each had a `dispositions:` map. Three mechanisms put the reviews there:
  - **The spec's own scenario or data-model edit made its review stale.** This was the first failure for nine of the 12. One was 038's one-line edit to a scenario, adding a path to a `§` reference. The rule was [079](../079-review-and-analysis-work-converges/spec.md)'s Resolved Question of 2026-11-04: a scenario's text is the requirement, so a change to it with no code change was when the code might no longer meet it.
  - **A sweep commit touching the spec's directory brought in the commit's code wherever the spec's plan listed it.** This was the only blocker for 010, 024 and 058: 067 task 19's commit `8651d466` edited `framework/constitution.md` and `framework/commands/analyze.md`.
  - **A commit naming no spec put its code in every open spec's review.** Examples were `0afc0c00` (`framework(bootstrap)`) and `9a268a5f` (`docs(agents)`).
- **Builds and code CI on spec-only pushes.** At `6158d7df`, `.github/workflows/framework-checks.yml` ran on every push and compiled the runtime before its artifact checks. `.github/workflows/generators.yml` filtered by path only on pull requests, so every push to `main` compiled the runtime too. `AGENTS.md` then had each run read before `done` or a tag, so a change to spec artifacts alone waited on two runtime builds. `runtime/tests/result_sizes.rs` read the checkout's own `specs/`, so a spec edit could change a `cargo test` result.
- **An adopter's report.** It covered a backend spec spanning three repositories, analyzed against eight rule files holding 141 rules. Closing the spec took three analyze rounds, two reviews, two follow-up tasks and about ten commits after its code was complete. The report named five problems:
  - Analyze's rule assessment surfaced new findings on each run against unchanged spec text: 10, then 8, then 4. Only findings were stored, so every run re-judged every rule. On the interactive path no primitive listed the rules, and nothing counted the rules a host left unjudged.
  - A comment-only edit to `data-model.md` forced a full review, which in turn staled the analysis.
  - The criterion-path and body-path families examined `done` specs only, so the gate's analysis was taken after flipping the spec to `done` and back. That was fixed on `main` in `63527403` and was not yet released.
  - A review rewriting `review.md` staled the analysis, since `review.md` is one of an analysis's subjects.

  The report matched `ductus-v0.59.11` (2026-10-17): its constitution still prescribed the flip, and it lacked `review-owed`. `6158d7df` was 1,083 commits past that tag.

## Behavior

- **A review is owed for code.** Code is any path outside the project's artifacts. A reopening whose own work changed only artifacts, a spec's scenarios and data model included, owes an analysis and no review. A review is still owed while the spec's record holds something only a review clears.
- **The checks follow the change.** A change confined to the project's artifacts owes the artifact checks, and no build, automated test, code lint or code CI. This is the constitution's blast-radius rule made concrete, and it ships to adopters.
- **The adopter gets it.** The changes ship in a `ductus-v<version>` release.

## Acceptance Criteria

- [ ] AC1: A reopening whose window changed the spec's own scenarios or data model, and nothing outside the project's artifacts, has `compute-review-scope` report `review-owed: false`, provided the review record holds nothing only a review clears.
- [ ] AC2: In that case `check-review-gate` blocks on no review check, `review is stale` included, and holds `done` only on the analysis. A fixture proves both directions: a contract-only change passes the review checks, and a change to a path outside the artifacts still blocks on a review.
- [ ] AC3: The framework self-audit's Family 19 reports no finding for a `done` spec whose only change since its review is to its own scenarios or data model.
- [ ] AC4: `framework/constitution.md` states the division (`/{project}:review` for code, `/{project}:analyze` for the project's artifacts) and the check-selection rule. `framework/reference/review.md` no longer says a scenario or data-model edit stales a review.
- [ ] AC5: 079's Resolved Question on scenario edits, and its AC4 annotation naming a stale digest, record the 2026-11-11 reversal and point here.
- [ ] AC6: A push to this repository whose changes are confined to the project's artifacts runs no `cargo test`, `cargo clippy`, `cargo fmt` or `cargo audit` in CI. `AGENTS.md`'s local-checks rule asks for no build and no wait on a code workflow for such a change.
- [ ] AC7: The changes reach adopters through a `ductus-v<version>` release, and the release ships `63527403`'s removal of the status flip.

## Applicable Rules

- `QUAL-CLAIM-001`: a gate that stops asking for a review must not read as one that ran it.

## Open Questions

- Sweep commits: when a commit several specs claim changes code, does each spec whose directory it touched owe a review of that code, or only the spec its subject names first?
- Commits naming no spec: their code currently lands in every open spec's review. Which review owes it, if any, and how does a review that owes none stay distinguishable from one that read it?
- Which paths are the project's artifacts for the check-selection rule: the spec root alone, or also `.ductus/`, `AGENTS.md`, `CLAUDE.md` and the constitution? In this repository `framework/` is the product, so it is code.
- Does a review that rewrites only `review.md` still stale the analysis (the adopter's fifth problem)?
- A code test that reads the spec corpus, like `runtime/tests/result_sizes.rs`, can fail on a spec-only change no CI runs. Does it move to a fixture, or into the artifact checks?
- Do per-rule verdicts and a count of unjudged rules on the interactive path (the adopter's first problem) belong in this spec or in one of their own?

## Resolved Questions

- **Does a reopening whose own work changed only a spec's scenarios or data model owe a review?** No (owner, 2026-11-11), reversing 079's answer of 2026-11-04. `/{project}:review` is responsible for code and `/{project}:analyze` for the project's artifacts. What this gives up: a requirement changed by hand in a scenario or data model, without a task, is checked against the code by nothing. A requirement added through `/{project}:amend`'s scenario route comes with a task, and that task's code change owes the review.
- **Where does this work live?** A new spec (owner, 2026-11-11), rather than a scenario on 079. 079's `spec.md` was 87,348 bytes, past the 50,000-byte read size, and 079 reopens either way, since its AC4 annotation names a stale digest as a reason a review is owed.
