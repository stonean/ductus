---
status: done
dependencies: [020-code-review, 024-rule-loader]
next-criterion: 10
---

# 025 — Rule-file opt-out via `.ductus/config.toml`

Add a narrow `.ductus/config.toml` `[[review.disabled-rule-files]]` opt-out so an adopter whose stack matches a rule file's surface — but whose project is not yet ready to enforce that file's rules — can exclude the file from `/ductus:review` loading with a recorded reason. Derivation per [024 — stack-aware rule-file loader](../024-rule-loader/spec.md) remains the default; this spec only adds the explicit override.

## Motivation

[Spec 024](../024-rule-loader/spec.md) makes rule-file selection derived: a project with frontend code automatically gets [`accessibility-frontend.md`](../../framework/rules/accessibility-frontend.md) and [`performance-frontend.md`](../../framework/rules/performance-frontend.md) enforced; a project with a backend API automatically gets [`api-backend.md`](../../framework/rules/api-backend.md) enforced. That default is correct — the alternative (every rule file is opt-in) is the silent author-discipline failure mode §design-principles' rule against features that depend on human diligence exists to prevent.

But derivation can't see intent. A team building an internal admin UI may not be ready to enforce WCAG AA. An early MVP may not have published an OpenAPI schema yet. An adopter migrating onto `ductus` mid-project will hit a backlog of rules they cannot triage all at once.

Without an override, those teams have three bad options:

1. **Pin the file in `.ductus/config.toml [pinned] files` and delete its content locally.** Defeats the purpose of shipping rules — the next `ductus` update can't push improvements to a file the adopter is now maintaining.
2. **Add a waiver for every finding.** Waivers are per-`(rule, file)` and intended for narrow, justified exceptions — using them to suppress an entire rule file means dozens of waiver entries that obscure the small number of waivers that actually mean something.
3. **Ignore `/ductus:review`'s `blocking: true` output.** Detaches the team from the gate the framework provides and removes the protection from rules they DO care about.

The fix: a deliberate, recorded, file-level opt-out. `.ductus/config.toml` already houses adopter-side state, one top-level section per concern, each documented by the spec that owns it (019's data model records the convention, and `AGENTS.md`'s entry on when a `.ductus/config.toml` change is cross-spec impact applies it). A new `[[review.disabled-rule-files]]` array-of-tables key lets adopters say "we know this file applies; we're not enforcing it yet; here's why." The reason is mandatory — without one, the entry is silently typo-able to a meaningless on/off switch.

## Acceptance Criteria

- [x] AC1: `.ductus/config.toml` accepts `[[review.disabled-rule-files]]` as an array-of-tables. Each entry has two required fields:
  - `file` — the **basename** of a file in `framework/rules/` (e.g., `"accessibility-frontend.md"`). Values containing path components (e.g., `"framework/rules/accessibility-frontend.md"` or `"rules/accessibility-frontend.md"`) are NOT special-cased — they fall through to the unknown-file warning (next AC) because no such basename exists in `framework/rules/`.
  - `reason` — a free-text justification (non-empty; trimmed length ≥ 16 **Unicode codepoints** — counted as scalar values, not bytes, so non-ASCII reasons like `"WCAG懸念 → Q3まで保留"` are evaluated by their visible length — that one is 15 codepoints, one short). An entry that fails the length check halts `/ductus:review` before any rule file is selected (AC4), naming the entry and the length, so the threshold can never be the cause of a missed enforcement or of a silent one.
- [x] AC2: `/ductus:review` reads `[[review.disabled-rule-files]]` during rule-file selection and skips any listed file regardless of stack detection. Skipped files emit a one-line stdout notice at the start of the run:

  ```text
  disabled-rule-file: <filename> — <reason> (.ductus/config.toml)
  ```

  The notice is the point: silent skipping is forbidden. Internal whitespace in `reason` (including newlines from TOML multi-line strings) is collapsed to single spaces in the notice — operators may use multi-line TOML strings for readability without breaking the one-line format.

  If a listed file would NOT have been selected by stack detection anyway, the entry is still processed and a distinct one-line notice fires:

  ```text
  disabled-rule-file (no-op): <filename> not selected by stack detection
  ```

  This is honest about state — the entry is currently a no-op, but becomes load-bearing if the project's stack changes later. It is not an error; operators may pre-list files for documentation purposes.
- [x] AC3: An entry whose `file` does not exist in `framework/rules/` produces a one-line warning (`unknown disabled-rule-file: <filename> (no such file in framework/rules/)`) but is not a fatal error — operators may temporarily list a file that has been renamed or moved. *(The warning's wording has since generalised: it reads `no such file in the rule-file directory`, because the directory is `specs/rules/` in an adopting project and `framework/rules/` here. A refactor, not a removal — no later spec countered this criterion, so the note records the rewording in place.)*
- [x] AC4: An entry that is not a table, that is missing `file` or `reason` or holds a non-string in either, or whose `reason` fails the minimum-length check, **halts `/ductus:review` before any rule file is selected**, naming the entry's index and the field that failed (`CFG-ENV-008`; see [a malformed disabled entry fails fast](scenarios/a-malformed-disabled-entry-fails-fast.md)). The entry is NOT auto-removed; the operator fixes it, because malformed entries are operator-authored state, not garbage for the framework to collect. The halt is a configuration error that stops the run, not a review finding, so it never sets `blocking`: a `blocking: true` result still means only "a MUST rule was violated", not "your `.ductus/config.toml` is malformed".
- [x] AC5: Duplicate entries (same `file` listed twice) emit a warning and only the first applies — same pattern as duplicate waivers.
- [x] AC6: `/ductus:status` surfaces the disabled list (when present) in the pipeline dashboard, so the override is visible at-a-glance and doesn't hide in `.ductus/config.toml`.
- [x] AC7: `/ductus:analyze` does NOT error on the new key. The key is a `.ductus/config.toml` extension owned by this spec, not a spec-frontmatter change.
- [x] AC8: The mechanism is uniform across all rule files. Adopters CAN disable [`security-backend.md`](../../framework/rules/security-backend.md) or [`security-frontend.md`](../../framework/rules/security-frontend.md) — the reason field is the audit trail. The framework does not enforce a "security files cannot be disabled" carve-out: enforcing it would require a hardcoded list of "real security" files that drifts from reality, and dropping security rules is a high-stakes decision that the reason field already makes visible. PR review and the operator's own policy are the safeguards, not the framework.
- [x] AC9: Documentation: the new `[[review.disabled-rule-files]]` schema is documented in this spec's body and reflected in `framework/commands/review.md` (per AGENTS.md's *a `.ductus/config.toml` change is cross-spec impact only when it modifies a key another spec already documented* entry — keys are documented in the spec that owns them and in the embedded command artifact, NOT retro-added to spec 019 or any earlier config spec). Spec 020 established this precedent for `[review] tech-stack-verified`; spec 025 follows the same pattern.

## Non-goals

- **Per-rule-id disabling.** The unit is the rule file. Finer granularity invites cherry-picking individual MUST violations, which is what waivers already exist for. Two mechanisms for the same problem create confusion about which to reach for.
- **Per-spec exemption (e.g., "skip `accessibility-frontend.md` for spec 042").** Defeats the gate's purpose — a rule that doesn't apply to spec 042 also doesn't apply to spec 043 next sprint. Project-wide disablement is honest; per-spec is laundering.
- **A "until-date" auto-expire.** Tempting but failure-mode-asymmetric — if the date is in the past, does `/ductus:review` re-enable enforcement silently (breaks a build the team wasn't expecting) or warn-only (everyone learns to ignore the warning)? Both options reintroduce the disciplines this spec is trying to remove. Operators who want a sunset commitment write it into the `reason` and remove the entry when ready. PR review enforces it.
- **Encoding "approval" or "second-author" requirements.** Same reasoning as the waiver design ([spec 020](../020-code-review/spec.md) Resolved questions): ductus has no runtime that can verify a second person approved; encoding it in the toml would be performative. Adopters whose policy requires co-authorization layer their own fields onto the entry — the §text-first-artifacts open-schema rule guarantees `/ductus:review` will not error on unknown fields.
- **A CLI shortcut for adding disabled entries (`/ductus:review --disable …`).** Frequency-asymmetric with `--waive`: waivers are added dozens of times across a project's life; a disabled-rule-file entry one to three times total. A shortcut for an action invoked twice doesn't earn its surface-area cost. Editing `.ductus/config.toml` directly is also the right path because it forces the operator to *see* the existing list at the moment of addition — the context where peer entries matter most. `/ductus:status` (AC) surfaces the list outside the CLI flow, so visibility is handled. Symmetry with `--waive` is not a sufficient reason on its own; the waiver design itself avoids gratuitous symmetry (no `--unwaive`, no `--list-waivers`). If real demand emerges, a follow-up spec adds it.

## Affected files

| File | Change | Strategy |
| --- | --- | --- |
| `framework/commands/review.md` | edit — §Inputs (add the new config key), §Behavior step 5 (consult the disabled list during selection), §Output (the new stdout notice) | update |
| `framework/commands/status.md` | edit — surface the disabled list in the pipeline dashboard | update |
| `framework/commands/analyze.md` | edit — extend `.ductus/config.toml` validation tolerance to accept the new key | update |
| `framework/constitution.md` | edit — §rules anchor (or wherever rule-file loading is described) gets a brief mention of the override mechanism | update |
| `framework/templates/project/ductus-toml.md` (or equivalent template/example) | edit — add a commented-out example block showing the schema | update |
| `scripts/lint-ductus-toml.sh` (if it exists) | edit — accept the new key | update |

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Minimum length for `reason`.** Keep the 16-character minimum. "Just non-empty" leaves the door open to `"x"`, `"todo"`, etc.; once those land in production they're political to remove. Real justifications (`"WCAG deferred to Q3"`, `"Internal admin UI"`, `"Migrating from foo"`) all clear the bar. PR review is a backstop, not a substitute — the framework is already explicit elsewhere (mandatory `reason`), so being explicit about minimum quality is consistent. An adopter who writes `"PoC only"` (8 chars) is told so at once: the entry halts the review naming its length, rather than quietly leaving the rule file enforced. AC updated to state this failure mode explicitly so future readers don't relitigate the number. *(Until the owner's 2026-10-19 decision that `CFG-ENV-008` stands, a short reason warned and the entry was skipped; [a malformed disabled entry fails fast](scenarios/a-malformed-disabled-entry-fails-fast.md) records the change.)*
- **Section placement: `[review]` vs `[rules]`.** Use `[review]`. The only consumer is `/ductus:review`, so adopters answer "what affects `/ductus:review`?" by reading one section. A `[rules]` section would be empty except for this single key — premature structure for hypothetical content. The existing `[review] tech-stack-verified` precedent matters: keys that ductus `/ductus:review` behavior live in `[review]`; follow the established pattern. From the adopter's perspective they're disabling enforcement *by the review gate*, not "rules" in the abstract — which matches the `[review]` framing. AC already uses `[[review.disabled-rule-files]]`; no body changes needed.
- **Exit code for malformed disabled entries.** A malformed entry never sets `blocking`. `/ductus:review`'s `blocking` is driven exclusively by MUST violations — overloading it with config-hygiene problems means `blocking: true` no longer unambiguously means "a MUST rule was violated", and that ambiguity defeats the single-purpose exit code. That reasoning still holds, and it is why the malformed entry is a configuration error that halts the run rather than a finding: the review does not complete, so it records no `blocking` at all, and the command exits non-zero as `CFG-ENV-008` requires of a value that does not parse. *(This answer originally kept malformed entries as warnings that skipped the entry, symmetric with malformed waivers, on the grounds that a skipped entry leaves the rule file enforced. The owner decided on 2026-10-19 that `CFG-ENV-008` stands — a present value that does not parse halts rather than being dropped — and [a malformed disabled entry fails fast](scenarios/a-malformed-disabled-entry-fails-fast.md) records it. Waivers live in `review.md`, not in configuration, and keep their warn-and-skip handling.)* AC updated to state the exit-code invariant explicitly.
- **CLI shortcut for adding disabled entries.** No shortcut. Moved to Non-goals so the decision is visible to future readers rather than left as a perpetual open question. Rationale captured in that section.
