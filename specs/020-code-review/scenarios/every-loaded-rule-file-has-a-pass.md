---
section: "Acceptance Criteria"
---

# Every-loaded-rule-file-has-a-pass

## Context

`/{project}:review` selects rule files by surface — every `*-backend.md` or `*-frontend.md` file for the project's surfaces, plus every `*-cross.md` — but `framework/reference/review.md`, which defines the five passes, names a rule file in only one of them: the security pass, "Walk every file in scope against the loaded security rules". The reuse, quality, efficiency, and simplicity passes are defined by their own heuristics and never say they apply a loaded rule file. So nothing in the procedure applies `accessibility-frontend.md`, `api-backend.md`, `concurrency-backend.md`, `configuration-cross.md`, `observability-backend.md`, `performance-backend.md`, `performance-frontend.md`, `quality-cross.md`, or `reliability-backend.md` to code, although review loads them and the same document says domain concerns "belong in additional rule files, not new passes". `quality-cross.md`'s header states that `/{project}:review`'s quality pass enforces its rules, which the quality pass's own definition does not say. Surfaced 2026-10-19 by the rule-file review; the owner chose to record it here and decide the design separately. Since 017's [a-secret-at-rest-is-flagged-in-every-project-shape](../../017-derive-dont-ask/scenarios/a-secret-at-rest-is-flagged-in-every-project-shape.md), a secret written in source or committed to version control is `configuration-cross.md` `CFG-ENV-013`'s, where it was `security-backend.md` `BE-DATA-003`'s, whose code check was the security pass's; until this design names a pass for `configuration-cross.md`, a `--security` run checks it only if the reviewer counts the file among the loaded security rules (owner, 2026-10-29).

## Behavior

- Every rule file `/{project}:review` loads is applied to the code in scope by a named pass, and `framework/commands/review.md` says which pass applies which file: a `security-*.md` file the security pass, `quality-cross.md` the quality pass, a `performance-*.md` file the efficiency pass, and every other loaded file, a project-authored file with an unrecognized suffix among them, the quality pass.
- A rule whose Verification is a design-time commitment, checked by `/{project}:analyze` against specs and plans, is applied by its pass only where the code in scope shows what its Statement governs.
- A rule file's header claim about the pass that enforces it agrees with `review.md`.
- The runtime's `performReview` extension carries each pass the rule files mapped to it, and the generated command copies are regenerated.

## Edge Cases

- A rule file whose rules are all design-time commitments may still have Statements a pass can check against code; its pass applies each only where the code shows the governed behavior, and records nothing for a Statement the code in scope does not reach.
- A project-authored rule file with an unrecognized suffix loads for every stack and goes to the quality pass, like any file no dimension fits.
- A run restricted to one pass (`--security`) applies only that pass's files; a loaded file mapped to a pass that did not run is named in the Summary as unapplied, never folded into a clean result.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Which design gives every loaded rule file a pass?** Each file is mapped to the pass whose dimension fits it — `security-*.md` to security, `quality-cross.md` to quality, `performance-*.md` to efficiency — and every other loaded file, the design-time files and an unrecognized suffix alike, to the quality pass, which applies a design-time Statement only where the code shows it (owner, 2026-11-12). At `ab2ad085` only `framework/reference/review.md` §Security pass named any rule file, so 9 of the 12 loaded files were applied by no pass; `quality-cross.md`'s header already names the quality pass, so the mapping changes no file of spec 036, and 020 alone takes the re-review. Rejected: (a) a single rules pass walking every file, which keeps task 15's yield routing in one pass but rewrites `quality-cross.md`'s header and five Verifications and 036's spec, data model and plan, and makes `--security` mean more than security; and (c) review loading no design-time file, which contradicts this scenario's first Behavior line and changes `discover-rule-files`, 022's surface, and 024's loader.
