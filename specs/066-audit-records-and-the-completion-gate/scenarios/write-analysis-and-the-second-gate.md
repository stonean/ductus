---
section: "The analyze record"
---

# Write-analysis and the second gate

## Context

The runtime half of [047's `analyze-run-durability`](../../047-analyze-findings-durability/scenarios/analyze-run-durability.md), recorded here because 022 owns the runtime and is where runtime rules stay findable.

The requirement is 047's: `/{project}:analyze` had no durable record, so `check-review-gate` could enforce the review half of the pipeline and nothing at all of the analyze half. What 022 owns is the shape of the primitive, the block's field set, the two new gate reasons, and the ninth `check-artifacts` family.

## Behavior

**`write-analysis`** — a new primitive, registered at all five sites (the CLI enum and its dispatch arm, the exec-path match arm, the `#[tool]`, `PRIMITIVE_REGISTRY`, and `framework/runtime-tools.txt`).

It writes `specs/NNN/analysis.md` and nothing else — the record in its frontmatter, the findings in a fixed body skeleton — rendering the whole file and writing it atomically on every run rather than splicing a block into an existing file, so there are no sibling keys to disturb. Spec 057 moved the record there from the spec's `analyze:` frontmatter block; `spec.md` is now only read, for the no-record-on-unparseable-frontmatter guard below.

The field set is the constitution's §text-first-artifacts *Analyze record — `analysis.md`* table.

**A bare `unexamined` total is not enough, and shipping one first was a mistake.** The field was introduced as a single integer, and asked what purpose that served, the honest answer was: not much. It distinguishes clean-verified from clean-partial and nothing more. The reasons are not equivalent — `not-a-live-claim` is a correct exclusion with nothing owed, `artifact-unreadable` is a real failure — and both increment the same counter, so `81 + 1` and `82 + 0` are indistinguishable. A total that conflates them repeats, one level down, the conflation the field exists to prevent. The information was never unavailable: the reason set is closed and documented, and the first implementation discarded it — the same shape as `apply-manifest` computing its substitution count and binding it to `_count`.

`unexamined-by-reason` carries the closed set, and the set splits into classes that call for opposite responses: **excluded by construction** (`not-a-live-claim`, `ships-to-adopter`, `root-absent` — correct, nothing owed) and **could not be read** (`target-missing`, `target-unparseable`, `no-readable-state` — a real gap in what the run saw; the data model's `write-analysis` table lists the reasons classified with them since).

`artifact-unreadable` began in the second class and no longer sits in either, because asking whether the class should block `done` showed it was the only member that should. At `done` it is now a **blocking finding**, not a skip. The rule it breaks — an unknown is never escalated into a defect — is about *other* files, where the defect is not this spec's; this one is the spec's own artifact that its own analysis could not read. And it is not merely principled: `scenario-open-questions` blocks at `done`, and an unreadable scenario contributed no questions and no finding, so a scenario carrying unresolved questions that would not parse passed the gate built to catch exactly that. A check that could not run wearing the costume of one that passed — *inside* the gate rather than beside it. Below `done` it remains a skip: the questions check is advisory there and an unreadable artifact mid-work is a state to report, not a gate to fail. The total is **summed from the breakdown** when one is supplied, so the two cannot disagree; that is the same discipline that derives `blocking` rather than accepting it.

`unexamined` is why this is not a copy of `ReviewBlock`. A clean analyze is two states — everything examined and clean, or something unexaminable and the rest clean — and `/{project}:analyze`'s own Unexamined-targets section already draws that line. A record carrying only finding counts would collapse it into the reassuring reading inside the artifact a later gate reads, which is the worst possible siting for that failure. `blocking` being **derived** rather than passed is the same discipline: a caller cannot record a clean gate over a dirty run.

A spec whose frontmatter does not deserialize gets **no** record. The value is parsed and discarded purely for that guard — the analysis would have hard-failed on such a spec, and writing a clean record into it inverts the mechanism.

**`check-review-gate`** gains checks 7 and 8, `not-analyzed` and `analyze-findings`, extracted into `analyze_gate_block` (the function was already at clippy's 100-line ceiling; the existing `stale_review_block` seam is the pattern). Two new `ReviewGateBlock` variants, `NotAnalyzed` and `AnalyzeFindings`, ordered after every review-record check.

The primitive keeps its name though it now gates on both commands. Renaming is five registration sites and a breaking MCP change to buy a more accurate noun; the name is documented as historical in `/{project}:implement` instead.

**Analyze-state drift** is judged by `/{project}:analyze` from the record it writes, not by the `check-artifacts` family it began as: read during detection, that family judged the record the run was about to replace, so its finding could never clear (058's `analyze-state-drift-judges-the-record-it-writes`). The new record always carries `last-run`, so the check reports a `done` spec whose new record is blocking. It mirrors `review-state-drift` on that gating state and deliberately not on the advisory count, where its sibling *does* check `should-violations`. That asymmetry is stated in both the check and the block's own documentation, because a reader who notices it will otherwise read it as an omission.

## Edge Cases

- **`skip_serializing_if` on `ReadSpecResult.analyze`.** Absent stays absent: `read-spec` on a feature with no `analysis.md` emits no `analyze` key rather than a defaulted block, so a consumer can distinguish "no record" from "an empty one".
- **A newline in `analyzed-at` or `analyzed-against`.** Flattened to a space, not rejected. `write-review` rejects the equivalent because its fields carry operator prose with intent to preserve; these two are a timestamp and a sha, where a newline is a caller defect with no legitimate reading and only a frontmatter injection to defuse.
- **A record written while the spec is reopened.** `criterion-path-existence` examines `done` specs only, so a spec mid-back-edge has a smaller skipped set than the same spec at `done` — measured on 022 itself, which reported 2 skipped at `done` and 0 while reopened. The record is honest either way (`analyzed-against` names the sha), but it describes a different subject. Write the record last, after the status is settled; the command's own step order already says so.
- **CRLF specs.** No splice remains to guard: `spec.md` is read and never written, and `analysis.md` is rendered whole on every run.
- **The gate's fixtures.** Every existing `check-review-gate` test fixture carried a `review:` block and no `analyze:` one, so all of them began failing the moment the check landed — which is the check working. (Spec 057 re-shaped those fixtures again, splitting both blocks out of the spec into sibling `review.md` and `analysis.md` files; the same fixtures, one file each.) They were given a clean analyze block; the new reasons got their own fixtures, including one asserting that a spec failing *both* gates is told about the review.

## Resolved Questions

- **Why not fold the analyze record into `write-review`, so one primitive writes both?** Because the two are written by different commands at different times, and a primitive that writes a block its caller did not compute is how one of them ends up stale. Sharing mechanism is the right amount — both primitives render their own record file whole, through the same subject digest, decisions list and atomic write: the mechanism is common, the claims are not.
- **Why is `blocking` derived rather than an argument?** So there is no call that records `blocking: false` alongside a non-zero `hard-fail`. `write-review` derives its own `blocking` from `must > 0` for the same reason. A field a caller can contradict is a field that will eventually be contradicted.
