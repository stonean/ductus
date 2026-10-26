# clarify — markdown-only reference

With no ductus runtime registered, the host walks the same contract with its own file tools (Read, Edit, Write) — no shell-pipeline substitution (§runtime-host-integration). The Gate table in `clarify.md` governs both paths.

**Where a named section lives.** This file is read alongside its command, `clarify.md`, which is already loaded whenever this one is. A section named below that has no heading in this file is in the command file, along with the numbered Instructions steps this file cites. A bare `§name` is a heading in this file or a constitution marker; a `§name` with another file's path on the same line is a heading in that file.

## Feature-targeted clarify (hot path: `draft` spec)

Read `spec.md`. If it does not exist, stop and report: "Spec does not exist. Run `/ductus:specify` first." Then perform the clarify gate defined in `.ductus/constitution.md` (§spec-requirements, §spec-lifecycle):

0. **Recompute dependencies (safety net).** Run `ductus derive-dependencies` (report-only by default; it walks every spec — there is no per-spec mode). If it reports drift, the `dependencies:` frontmatter is stale from uncommitted body edits; surface that and recommend committing (the pre-commit hook syncs it) or running `ductus derive-dependencies --write` manually. Do not pass `--write` from this command — evaluate dependency readiness against the current frontmatter and note the caveat.

1. **Resolve open questions one at a time** — process each open question individually in sequence:
   1. Display the question with its full context.
   2. Propose an answer with rationale, or ask the user to decide. **When the question turns on existing reality — how current code behaves, what a schema, interface, or dev database holds — consult the specific source that settles it and ground the proposed answer in what you found, citing it (`path:line`, the query, or the command). Do not resolve a factual question about existing code by conjecture; when no reachable source can settle it, say so and resolve it as an explicit assumption or leave it open (§grounding).**
   3. Wait for the user to review, discuss, refine, or approve the resolution.
   4. Only after the user confirms, move the question from `## Open Questions` to `## Resolved Questions` and proceed to the next one.
   5. If the user wants to skip a question, move to the next and revisit skipped questions at the end.
   6. If resolving one question invalidates or changes another, note the impact when presenting the affected question.
   - Do NOT present multiple questions at once. Do NOT batch resolutions.
   - Process only items in `## Open Questions`. Items already in `## Resolved Questions` are never re-walked.
2. **Enumerate edge cases** — for each behavior, identify what happens with empty inputs, missing data, duplicates, boundary values, and concurrent access.
3. **Confirm error scenarios** — verify every failure mode has a defined behavior (HTTP status, error code, message). Flag gaps.
4. **Verify acceptance criteria** — check each is concrete, testable, and unambiguous. Rewrite vague ones. Flag missing criteria.
5. **Check dependency readiness** — for each entry in this spec's frontmatter `dependencies` list, read that spec's frontmatter `status` field. Confirm each dependency is at `clarified` or later. Flag blockers.
6. **Cross-spec impact check** — list every sibling spec referenced by inline markdown link in the body (the union the dependency scan already computed). Ask: "Do any of these referenced specs need an update because of decisions made here?" If yes, the §cross-spec-impact rule applies — the change goes in the affected spec as a new acceptance criterion or scenario, with a back-link to this spec. This step is informational; it does not block the transition.

After the review:

- Update the spec body with resolved questions and any new edge cases or acceptance criteria.
- **Label the acceptance criteria** (primitive: `label-criteria`) — every criterion added during this clarification gets its stable `AC{n}:` label written between the checkbox and its text, and `next-criterion` is updated in the frontmatter. Existing labels are never renumbered; the next label is `max(highest label in body, next-criterion)`, never `max(body) + 1`, so a criterion deleted earlier never has its label handed to a different requirement (§text-first-artifacts, spec 013).
- If questions remain that need user input, list them and keep `status` at `draft`.
- If all open questions are resolved, run the validation gate before proposing the status transition:
  - All open questions are resolved (none remain in the Open Questions section)
  - Acceptance criteria are concrete and testable — no empty placeholders
  - Dependencies are at `clarified` or later
  - The modified spec file passes `npx markdownlint-cli2`
- If any check fails, report the specific failures and do not propose the transition. The user fixes the issues and re-runs the command.
- If all checks pass, present a summary of changes and ask the user to approve the transition to `clarified`. Do not update the status until the user confirms.
- On confirmation, update the frontmatter `status` field from `draft` to `clarified`.
- Display the next step: "Run `/ductus:plan` to create the technical plan."

## Recovery path: non-`draft` spec with open questions

Triggered only when the gate sees `(status ∈ {clarified, planned, in-progress}) && open-question count ≥ 1`. This state should not occur via normal usage — `/ductus:amend` reverts a spec to `draft` whenever it records a new open question on a non-`draft` spec — but it can arise from a manual frontmatter edit or a spec migrated from another tool.

Before mutating anything, surface the inconsistency to the user:

1. **Display the inconsistency:**
   - Current `status` value.
   - Count and titles of entries in `## Open Questions`.
   - Existence and last-modified timestamp of `plan.md`, `tasks.md`, and `data-model.md` in the feature directory. Omit files that do not exist.
   - The list of files in `specs/{feature}/scenarios/` if that directory exists.
2. **Prompt the user:**
   > Spec is `{status}` but has {N} unresolved open questions in the body — this state usually arises from a manual frontmatter edit. Revert status to `draft` and walk the questions?
3. **Confirm** — update the frontmatter `status` field to `draft` (the `set-status` primitive on the runtime path; a direct frontmatter edit otherwise), then run the **Feature-targeted clarify (hot path: `draft` spec)** procedure above (including the dependency-readiness check; the post-revert walk runs the same checks as a normal `draft` clarify). On successful resolution, the spec advances back to `clarified`. Downstream artifacts (`plan.md`, `tasks.md`, `data-model.md`, scenario files) are not deleted or rewritten by this command.
4. **Decline** — exit without modifying any file. The spec retains its inconsistent state and open questions remain in `## Open Questions`. The next `/ductus:clarify` invocation offers the same prompt — the system surfaces the inconsistency on every clarify attempt rather than silently advancing.

`## Resolved Questions` is never re-walked even on the recovery path; only items in `## Open Questions` are processed.

## Scenario open-question report

Rendered by a **feature-targeted** run in whichever gate branch it takes, whenever the feature's scenarios carry unresolved questions. It reports; it never resolves, never writes, and never gates.

**Deriving the list.** On the runtime path it is `read-spec`'s `scenario-open-questions` field — already loaded at step 2, so no extra call and no extra read. On the markdown-only path, read the `## Open Questions` section of each file under `specs/{feature}/scenarios/` and count its entries by the same rule the Gate uses (placeholder lines such as `*None — all resolved.*` are not entries). Read those sections only, per the Scope Boundaries carve-out. Enumerate the directory with the shared scenario ordering — case-insensitive with a raw-byte tiebreak — so the two paths list the same scenarios in the same order. A scenario file that cannot be read or parsed contributes nothing and is not escalated into a defect: nothing can be proven about a file that will not parse. It is still named, in both paths — on the runtime path from `read-spec`'s `scenario-files-unreadable`, on the markdown-only path from whichever files you could not read — appended as `(N scenario file(s) could not be read: {slug}, … — these were not examined)`. Reporting zero questions over a scenario nothing could read would assert exactly what was never checked.

**Wording.** Name every carrying scenario and its count — no cap, since a truncated list reads as "these are the ones that need attention" while hiding others — then the command that resolves them:

```text
{N} unresolved question(s) in {M} scenario(s): {scenario} ({count}), {scenario} ({count}).
These do not block this transition; they block `done`.
Run /ductus:target {feature}/<scenario> then /ductus:clarify on each to resolve them.
```

**Suppression.** When no scenario carries a question the report is omitted **entirely** — not rendered as "0 outstanding". A feature with clean scenarios reads exactly as it did before this report existed.

**Placement.** After the branch's own message. On the two terminating branches (`already {status}`, `done`) it follows the stop message and neither branch gains a write — both keep their guarantee of modifying no file. On the advancing branch it precedes the "Run `/ductus:plan`" line, so the outstanding work is read before the affirmative next step rather than after it. That ordering is the point of the report: the pre-`done` gate blocks on exactly these questions, and a command holding the signal must not answer an unresolved spec with an affirmative next step alone.

**Not a gate.** `draft → clarified` still advances carrying scenario questions. The mechanized block stays at `done` (`check-review-gate`), and §readiness-check already counts the spec body's questions *and* those carried by any scenario at `planned → implement`. Adding a second gate here would change what `clarified` means.

## Scenario-targeted clarify

1. **Resolve open questions one at a time** — process each open question in the scenario's `## Open Questions` section individually in sequence:
   1. Display the question with its full context.
   2. Propose an answer with rationale, or ask the user to decide. **When the question turns on existing reality — how current code behaves, what a schema, interface, or dev database holds — consult the specific source that settles it and ground the proposed answer in what you found, citing it (`path:line`, the query, or the command). Do not resolve a factual question about existing code by conjecture; when no reachable source can settle it, say so and resolve it as an explicit assumption or leave it open (§grounding).**
   3. Wait for the user to review, discuss, refine, or approve the resolution.
   4. Only after the user confirms, move the question to Resolved Questions and proceed to the next one.
   5. If the user wants to skip a question, move to the next and revisit skipped questions at the end.
   - Do NOT present multiple questions at once. Do NOT batch resolutions.
2. **Enumerate edge cases** — identify edge cases specific to the scenario's behavior (empty inputs, missing data, boundary values, concurrent access).
3. **Verify behavior section** — confirm the scenario's Behavior section is unambiguous and complete.

After the review:

- Move resolved questions from `## Open Questions` to `## Resolved Questions` with their answers.
- Add any new edge cases to the scenario's `## Edge Cases` section.
- If questions remain that need user input, list them.
- The scenario does not have its own status field — resolution is complete when all open questions are removed from the Open Questions section.
- Run `npx markdownlint-cli2` on the modified file.
- Read the parent spec's frontmatter `status` field. Display: "Scenario clarification complete." and suggest `/ductus:implement` if the parent spec is `planned` or `in-progress` (both states are accepted by `/ductus:implement`'s gate). For other parent-spec states (`draft`, `clarified`, `done`), display the completion message without a next-step suggestion — the parent spec's own pipeline state determines what comes next.
