---
section: "Follow-on scenarios"
---

# Family-10-migration-coverage

## Context

[`/audit`](../../../framework/commands/audit.md) shipped in spec 026 with nine family checks (1–9). Spec [027 — Bootstrap Migration Registry](../../027-bootstrap-migration-registry/spec.md) extends `/audit` with a tenth family — `scripts/audit/migration-coverage.sh` — that verifies the new `framework/migrations.toml` registry is consistent with its adjacent `framework/migrations/{id}.md` procedure files and with current `framework/` state.

This scenario records the extension on 026 per §cross-spec-impact: spec 026 was at `status: done` when 027 was written, so the Family 10 addition lands as a scenario here rather than a body edit, with a back-link to its driving spec.

Origin: spec 027 plan + tasks, 2026-05-21.

## Behavior

Family 10 follows the same contract as Families 1–9: stdout findings, exit 0/1, read-only, idempotent. Its checks run unconditionally on every `bash scripts/audit/run-all.sh` invocation, and they are the ones `specs/027-bootstrap-migration-registry/spec.md` §Audit Family lists and `scripts/audit/migration-coverage.sh`'s header comment describes. This scenario names no copy of that list, so a check 027 adds leaves it true (owner, 2026-11-18).

Family 10 is invoked from `scripts/audit/run-all.sh` immediately after Family 9 and exits with the OR of its checks' findings. Its findings format matches the rest: `migration-coverage | {location} | {message} | {suggested-fix}`.

10b's archived half reads the format the archived entries keep in `CHANGELOG.md`: a `### {id} (…)` heading, then a `**Target paths:**` line of backticked paths (027's review of 2026-11-17, which archived five entries past their `sunset_after`).

## Edge Cases

- **Procedure file with a typo'd id**. Family 10 surfaces both checks: 10a (orphan procedure file) AND 10c (broken procedure_file reference), unless the typo is in both locations consistently — in which case 10b might still catch it if the path doesn't exist. Multiple findings on the same root cause are acceptable; they all point at the same fix.
- **Registry entry with no `procedure_file` field**. Currently treated as `procedure_file = ""`, which fails 10c (the empty string path doesn't exist). Acceptable v1 behavior; a future enhancement could enforce the field's presence schematically.
- **Glob in `target_paths` matches a pinned file in adopter context**. Not Family 10's concern — pinned-file handling lives in the bootstrap loop and procedure files. Family 10 only sees the registry's text.
- **Two registry entries reference the same `procedure_file`**. When they share an `id` too, as a copied entry does, Family 10's duplicate-id check and the bootstrap loop's duplicate-id guard each catch it (027 task 19). With distinct ids, each entry passes 10c, and neither check compares procedure files; a further Family 10 check could enforce procedure-file uniqueness, captured as future work.
- **A new family check ships.** It is a hard gate from its first run. This bullet originally said a new family inherits soft-launch (`continue-on-error: true`) per the [audit-ci-hard-gate](audit-ci-hard-gate.md) scenario's contract, and that was already untrue when it was written: the flip landed in `5732c6de` on 2026-05-17 and this scenario was recorded on 2026-05-21, so no per-family soft-launch regime existed for Family 10 to inherit. `continue-on-error` appears nowhere in `.github/workflows/` today, and every family since 10 has shipped straight into the gate.
- **A path the family did not write as a literal is contained first.** Under the rule the plan's decision *Every family contains the paths it did not write as a literal* states once (`BE-INPUT-004`), any path of these kinds resolving outside the repository is reported and never touched: a `procedure_file` or `target_paths` entry, a glob match, a procedure file.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Should Family 10 enforce procedure-file uniqueness (no two entries share a `procedure_file`)?** **Resolved with a condition: not enforced in v1.** The bootstrap loop's duplicate-id guard already makes shared procedure files inconsistent at read time, so the drift Family 10 would catch is caught one layer down and the practical risk is small; a further Family 10 check stays captured as future work (see the Edge Cases entry above). Both layers catch the copied entry, which shares its `id`, and neither catches two distinct ids naming one file (027 task 19). *Trigger to revisit:* a real registry lands two entries sharing a `procedure_file` and the duplicate-id guard does not surface it legibly.
- **CHANGELOG.md archived-entry format**. **Resolved with a condition: implement the parser against a real example, not a speculative one.** The first sunset commit establishes the heading shape and the `target_paths` placement; writing the parser before that commit would encode a guess. Tracked meanwhile via spec 027's known-limitations section. *Trigger to revisit:* the first sunset commit lands, fixing the archived-entry format on disk.
