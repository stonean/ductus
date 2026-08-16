# 024 — Stack-aware rule-file loader for `/ductus:review` Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Rename `configuration.md` to `configuration-cross.md`

- [x] `git mv framework/rules/configuration.md framework/rules/configuration-cross.md`
- [x] Confirm rule IDs inside the file are unchanged (`CFG-CONST-*`, `CFG-ENV-*`)
- [x] Done when `framework/rules/` lists `configuration-cross.md` and no longer lists `configuration.md`

## 2. Create `scripts/lint-rule-filenames.sh`

- [x] Write a bash script that iterates `framework/rules/*.md`, fails non-zero on any basename not ending in `-backend.md`, `-frontend.md`, or `-cross.md`
- [x] Error message names all three valid suffixes
- [x] Make the script executable (`chmod +x`)
- [x] Run locally — passes after task 1 completes
- [x] Done when `bash scripts/lint-rule-filenames.sh` exits `0` against the current `framework/rules/` directory

## 3. Wire the lint script into CI

- [x] Edit `.github/workflows/markdown-only-pipeline.yml`: add a new step in the `markdown-only` job alongside the existing `lint-*.sh` invocations
- [x] Step name: `(g) Rule filename suffix lint` (or next available letter)
- [x] Step command: `bash scripts/lint-rule-filenames.sh`
- [x] Done when the workflow YAML parses cleanly and the new step appears in the step list

## 4. Update `framework/constitution.md` §rules

- [x] Add a new subsection `#### Filename suffix` before `#### Lifecycle` (around line 285)
- [x] Body: state the closed suffix set (`-backend.md`, `-frontend.md`, `-cross.md`), name the surface each selects, reference `scripts/lint-rule-filenames.sh` as ductus-side enforcement and the runtime warning as the adopter-side safety net
- [x] Update line 173 reference: `framework/rules/configuration.md` → `framework/rules/configuration-cross.md`
- [x] Update line 179 reference: same change
- [x] Done when the new subsection renders cleanly and no live references to `configuration.md` (without `-cross`) remain in the constitution

## 5. Rewrite `/ductus:review` rule-file selection

- [x] In `framework/commands/review.md`, rewrite §Behavior step 5 (lines 88–90) to describe the suffix-based discovery procedure: iterate `framework/rules/*.md`, classify each by basename suffix, filter by detected stack (keeping cross-cutting and unrecognized-suffix files), emit the `loading rule files:` notice plus any per-file unrecognized-suffix warnings
- [x] Rewrite §Behavior step 2 (§Load rules, lines 92–103) to describe the discovery output rather than the hardcoded `security-backend.md` / `security-frontend.md` list. The "Any other `framework/rules/*.md` referenced from `AGENTS.md`" line is removed (replaced by the new discovery)
- [x] Add the unrecognized-suffix warning text under §Blocking message (or a new §Notices section): `rule file <name> has unrecognized suffix — loading for all stacks; rename to -backend.md, -frontend.md, or -cross.md`
- [x] Rewrite §Notes for adopters (lines 415–426): replace the bullet about "automatically loads anything in `framework/rules/` referenced from `AGENTS.md`" with: (a) files inside `framework/rules/` are auto-discovered by directory walk — no `AGENTS.md` reference required; (b) the `AGENTS.md` fallback survives only for adopter-local rule files placed outside `framework/rules/`
- [x] Done when the hardcoded names `security-backend.md` and `security-frontend.md` no longer appear in `framework/commands/review.md` as selection criteria

## 6. Rewrite `/ductus:analyze` rule-file discovery

- [x] In `framework/commands/analyze.md`, rewrite §Rules (around line 133) to apply the shared suffix-based discovery: iterate the rule-file directory, classify by basename suffix, load every discovered file (no stack filtering)
- [x] Remove the closed list at lines 137–141 (`specs/rules/security-backend.md`, `specs/rules/security-frontend.md`, `specs/configuration.md`)
- [x] Add a sentence clarifying: "`/ductus:analyze` loads every discovered rule file regardless of detected stack — citation verification spans surfaces."
- [x] Emit the same `loading rule files:` notice and unrecognized-suffix warnings as `/ductus:review`
- [x] Done when no hardcoded filename list remains under §Rules

## 7. Sweep `configuration.md` references in framework command files

- [x] `framework/commands/implement.md` line 49: `framework/rules/configuration.md` → `framework/rules/configuration-cross.md`
- [x] `framework/commands/groom.md` line 43: `specs/configuration.md` → `specs/rules/configuration-cross.md`
- [x] Done when `grep -rn 'configuration\.md' framework/ scripts/ docs/ README.md AGENTS.md` returns no hits outside spec 024 itself

## 8. Update bootstrap map and add migration

- [x] `framework/bootstrap/ductus.md` line 393: source `framework/rules/configuration-cross.md`, destination `specs/rules/configuration-cross.md`
- [x] Add a one-pass migration check in the same file, modeled on the existing `spec-and-plan.md` cleanup from spec 023: on each `/ductus` invocation, detect any `specs/configuration.md` in the adopting project and offer to rename it to `specs/rules/configuration-cross.md`; emit a one-line notice
- [x] Done when the bootstrap doc shows the new map row and the migration step

## 9. Record the rename in `specs/README.md`

- [x] Under §Past Renames, add an entry naming the rename (`configuration.md` → `configuration-cross.md`, spec 024, 2026-05-17), the reason (closed-suffix rule-file naming policy), and the rule-ID stability note (`CFG-CONST-*`, `CFG-ENV-*` are content-anchored and unchanged).
- [x] Done when the entry appears in the file

## 10. Regenerate `.claude/commands/ductus/*.md`

- [x] Run `scripts/gen-claude-commands.sh`
- [x] Confirm `.claude/commands/ductus/review.md`, `.claude/commands/ductus/analyze.md`, `.claude/commands/ductus/implement.md`, `.claude/commands/ductus/groom.md` reflect the updated framework sources
- [x] Done when `git diff --exit-code` against the generated directory shows no unexpected drift

## 11. Run repo-local validation

- [x] `bash scripts/lint-rule-filenames.sh` — exits `0`
- [x] `npx markdownlint-cli2` — passes
- [x] `bash scripts/lint-tool-coverage.sh` — passes
- [x] `bash scripts/lint-frontmatter.sh` — passes
- [x] `bash scripts/gen-spec-deps.sh --dry-run` — clean
- [x] Done when every check exits `0`

## 12. Run `/ductus:analyze` and `/ductus:review` against this spec

- [x] `/ductus:analyze` against `024-rule-loader` — no blocking findings; the new discovery loads every `framework/rules/*.md` file under the closed-suffix policy
- [x] `/ductus:review` against `024-rule-loader` — `review.md` records 0 MUST violations; the `loading rule files:` notice lists every file in `framework/rules/` (ductus is a full-stack repo for review purposes)
- [x] Done when both commands complete with `blocking: false` and no MUST violations
