---
section: "Acceptance Criteria"
---

# Every-loaded-rule-file-has-a-pass

## Context

`/{project}:review` selects rule files by surface — every `*-backend.md` or `*-frontend.md` file for the project's surfaces, plus every `*-cross.md` — but `framework/commands/review.md` names a rule file in only one of its five passes: the security pass "walks every file in scope against the loaded security rules". The reuse, quality, efficiency, and simplicity passes are defined by their own heuristics and never say they apply a loaded rule file. So nothing in the procedure applies `accessibility-frontend.md`, `api-backend.md`, `concurrency-backend.md`, `configuration-cross.md`, `observability-backend.md`, `performance-backend.md`, `performance-frontend.md`, `quality-cross.md`, or `reliability-backend.md` to code, although review loads them and the same document says domain concerns "belong in additional rule files, not new passes". `quality-cross.md`'s header states that `/{project}:review`'s quality pass enforces its rules, which the quality pass's own definition does not say. Surfaced 2026-10-19 by the rule-file review; the owner chose to record it here and decide the design separately.

## Behavior

- Every rule file `/{project}:review` loads is applied to the code in scope by a named pass, and `framework/commands/review.md` says which pass applies which file.
- A rule file's header claim about the pass that enforces it agrees with `review.md`.
- The runtime's `performReview` extension carries whatever the chosen design hands each pass, and the generated command copies are regenerated.

## Edge Cases

- A rule file whose rules are all design-time commitments (checked by `/{project}:analyze` against specs and plans) may still have Statements a pass can check against code; the design says whether review applies them.
- A project-authored rule file with an unrecognized suffix loads for every stack and needs a pass like any other.

## Open Questions

- Which design: (a) the security pass becomes a rules pass that walks every loaded rule file, with the other four passes keeping their dimension heuristics; (b) each rule file is mapped to the pass whose dimension fits it (`security-*.md` to security, `quality-cross.md` to quality, `performance-*.md` to efficiency, and so on), with a rule for files that fit none; or (c) another split?

## Resolved Questions

*None yet.*
