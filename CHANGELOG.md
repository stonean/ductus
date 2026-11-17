# CHANGELOG

Adopter-facing record of framework conventions that have been removed.
Each entry is a recipe for adopters who skipped past the active sunset
window and need to apply the removal manually. See spec
[027 — Bootstrap Migration Registry](specs/027-bootstrap-migration-registry/spec.md)
for the governing pipeline.

The runtime crate's own release notes live at [`runtime/CHANGELOG.md`](runtime/CHANGELOG.md).

## Archived migrations

Entries here left `framework/migrations.toml` after their sunset window closed. Pre-run Migrations skips an entry once the release reaches its `sunset_after`, so an adopter far enough behind to still need one applies the recipe by hand. Each keeps its **Target paths**, which `/audit` Family 10's check 10b holds absent from `framework/`, as it does an active entry's. `skills-to-workflows` and `workflow-filename-rename` are **subsumed** by `workflows-sunset` (spec 043), archived below them.

### skills-to-workflows (introduced 0.2.0, sunset after 0.10.0)

**Summary:** Remove legacy `{config_dir}/commands/{project}/skills/` directory left over from the `skills/` → `workflows/` rename.

**Target paths:** `{config_dir}/commands/{project}/skills/`, `framework/skills/`

The rename (introduced by spec 010, delivered alongside spec 005's reopen) moved every workflow file into a new `workflows/` directory. The old `skills/` tree is unreferenced and safe to remove on any adopter project that scaffolded prior to the rename.

1. **Idempotency check.** If `{config_dir}/commands/{project}/skills/` does not exist, exit silently.
2. **Pinned-files check.** If the directory exists and **is** listed in the active config file's `pinned.files` (path comparison after placeholder resolution), leave it alone and report `pinned (kept): {config_dir}/commands/{project}/skills/` in the post-scaffolding summary. Exit without further action.
3. **Recursive delete.** Otherwise, recursively delete the directory.
4. **Summary line.** Report `removed (legacy skills/ directory): {config_dir}/commands/{project}/skills/` in the post-scaffolding summary.

The cleanup is unconditional once the directory is detected and unpinned — the `workflows/` directory replaced it on every `/govern` run after the rename, so any remaining `skills/` tree is necessarily stale.

### workflow-filename-rename (introduced 0.2.0, sunset after 0.10.0)

**Summary:** Remove legacy `{category}-{language}-{tool}.md` workflow files left over after the post-005 filename simplification to `{tool}.md`.

**Target paths:** `{config_dir}/commands/{project}/workflows/format-go-gofmt.md`, `{config_dir}/commands/{project}/workflows/format-python-black.md`, `{config_dir}/commands/{project}/workflows/format-typescript-prettier.md`, `{config_dir}/commands/{project}/workflows/lint-go-golangci-lint.md`, `{config_dir}/commands/{project}/workflows/lint-python-ruff.md`, `{config_dir}/commands/{project}/workflows/lint-typescript-eslint.md`, `{config_dir}/commands/{project}/workflows/test-go-gotest.md`, `{config_dir}/commands/{project}/workflows/test-python-pytest.md`, `{config_dir}/commands/{project}/workflows/test-typescript-vitest.md`, `framework/workflows/format-go-gofmt.md`, `framework/workflows/format-python-black.md`, `framework/workflows/format-typescript-prettier.md`, `framework/workflows/lint-go-golangci-lint.md`, `framework/workflows/lint-python-ruff.md`, `framework/workflows/lint-typescript-eslint.md`, `framework/workflows/test-go-gotest.md`, `framework/workflows/test-python-pytest.md`, `framework/workflows/test-typescript-vitest.md`

In `{config_dir}/commands/{project}/workflows/`, delete any file whose name appears in this exact set:

- `format-go-gofmt.md`
- `format-python-black.md`
- `format-typescript-prettier.md`
- `lint-go-golangci-lint.md`
- `lint-python-ruff.md`
- `lint-typescript-eslint.md`
- `test-go-gotest.md`
- `test-python-pytest.md`
- `test-typescript-vitest.md`

1. **Idempotency check.** For each filename in the set above, check whether the file exists in `{config_dir}/commands/{project}/workflows/`. If none of the files exist, exit silently.
2. **Per-file pinned check.** For each existing match, skip it if listed in the active config file's `pinned.files`. Adopters who customized a legacy file and want to keep it can pin its destination path.
3. **Delete.** Delete each remaining match.
4. **Summary line.** For each deletion, report `removed (legacy workflow): {filename}` in the post-scaffolding summary. Omit the line when nothing was deleted.

The check is by exact filename match against the set above; custom user files (e.g., `pytest-fast.md`) are never affected because they aren't in the set.

### gitignore-marker-rename (introduced 0.2.0, sunset after 0.10.0)

**Summary:** Replace `# Governance` marker in `.gitignore` with `# govern`.

**Target paths:** `.gitignore`

1. **Idempotency check.** If the project's `.gitignore` does not contain a `# Governance` line, or already contains a `# govern` line, exit silently.
2. **Replace.** Replace the first occurrence of `# Governance` with `# govern` in `.gitignore`. The marker check used by the `.gitignore` merge step elsewhere in the bootstrap uses the new spelling, so this rename keeps idempotency intact for that step.
3. **Summary line.** Report `migrated .gitignore marker: # Governance → # govern` in the post-scaffolding output.

### governance-config-rename (introduced 0.2.0, sunset after 0.10.0)

**Summary:** Rename `.governance.toml` → `.govern.toml` in the project root.

**Target paths:** `.governance.toml`

1. **Idempotency check.** If `.governance.toml` does not exist in the project root, exit silently. If both `.governance.toml` and `.govern.toml` exist, leave them alone and warn `Both .governance.toml and .govern.toml exist; remove the legacy file to silence this warning.` Then exit without modifying either file.
2. **Rename.** Rename `.governance.toml` to `.govern.toml` via `mv` (or `git mv` when the source is tracked).
3. **Summary line.** Report `migrated config: .governance.toml → .govern.toml` in the post-scaffolding output.

### spec-and-plan-sunset (introduced 0.5.0, sunset after 0.10.0)

**Summary:** Rename `specs/*/spec-and-plan.md` → `specs/*/spec.md` (lightweight-track sunset).

**Target paths:** `specs/*/spec-and-plan.md`, `framework/templates/spec/spec-and-plan.md`

The lightweight track was removed in spec 023. Adopters who scaffolded under the prior dual-template model may still have `spec-and-plan.md` files at any non-`done` status under `specs/`. Pipeline commands now look for `spec.md` only — those files would fail the "spec does not exist" gate on the next command.

1. **Idempotency check.** Walk `specs/*/spec-and-plan.md`. If no matches, exit silently.
2. **Per-file prompt.** For each match, prompt the user with the source path and the proposed destination:

   ```text
   Found legacy spec-and-plan.md: specs/{NNN-feature}/spec-and-plan.md
   Rename to specs/{NNN-feature}/spec.md? (Y/n)
   ```

3. **Per-file action.** On confirm, rename via `mv`. On decline, emit a warning and continue with the next match:

   ```text
   warning: specs/{NNN-feature}/spec-and-plan.md kept; pipeline commands will fail on this feature until renamed manually.
   ```

4. **Summary line.** When N > 0 files were renamed, report `migrated N spec-and-plan.md files` in the post-scaffolding output. Omit the line when N = 0.

Files at `status: done` are also renamed (the rename is a pure filename change — body and frontmatter are byte-identical, so it qualifies as a mechanical edit under [§spec-lifecycle](framework/constitution.md#spec-lifecycle) and the spec stays `done`).

### rule-files-relocate (introduced 0.6.0, sunset after 0.10.0)

**Summary:** Relocate rule files from `specs/` root to `specs/rules/` (subsumes `configuration.md` → `configuration-cross.md`).

**Target paths:** `specs/configuration.md`, `specs/*-backend.md`, `specs/*-frontend.md`, `specs/*-cross.md`

Rule files installed in adopter projects live under `specs/rules/{rule-set}.md` so the top-level `specs/` directory stays focused on feature spec directories (`specs/NNN-*/`), `inbox.md`, `README.md`, and the seed system files. Adopters scaffolded before this change have rule files at `specs/{rule-set}.md`, which `/{project}:review` and `/{project}:analyze` no longer discover (the discovery walk targets `specs/rules/*.md`).

This migration subsumes the earlier `configuration.md` → `configuration-cross.md` rename (closed-suffix policy, spec 024): when the legacy `specs/configuration.md` is found, it is renamed **and** relocated in one move.

1. **Idempotency check.** Walk the top level of `specs/` for files matching either:
   - `configuration.md` (pre-closed-suffix layout)
   - `*-backend.md`, `*-frontend.md`, or `*-cross.md` (closed-suffix rule files at the `specs/` root)

   For each match, compute the destination:
   - `specs/configuration.md` → `specs/rules/configuration-cross.md`
   - otherwise → `specs/rules/{basename}`

   Skip any source listed in `.govern.toml` `pinned.files` (path comparison after placeholder resolution) and emit one line: `warning: {source} is pinned; leaving in place — /{project}:review and /{project}:analyze will not discover it until moved manually.`

   Skip any source whose destination already exists in `specs/rules/` and emit one line: `warning: {destination} already exists; skipping relocation of {source}.`

   If no eligible moves remain, exit silently.

2. **Batch prompt.** If at least one eligible move remains, prompt once:

   ```text
   Found N legacy rule file(s) at the specs/ root.
   Move to specs/rules/? (Y/n)
   ```

3. **Action.** On confirm, create `specs/rules/` (if missing) and rename each eligible file via `mv` (or `git mv` when the source is tracked, so the rename is recorded). On decline, emit one warning per skipped file: `warning: {source} kept; /{project}:review and /{project}:analyze will not discover it until moved manually.`

4. **Summary line.** When N > 0 files were relocated, report `relocated N rule file(s) → specs/rules/` in the post-scaffolding output. Omit the line otherwise.

Rule IDs (`BE-AUTHN-*`, `FE-XSS-*`, `CFG-CONST-*`, etc.) are content-anchored and unchanged by the relocation; every existing citation continues to resolve.

### workflows-sunset (introduced 0.23.0, sunset after 0.25.0)

**Summary:** Remove the scaffolded workflow slash commands, the synced `workflows/registry.json`, and the `[workflows]` config section — the workflows feature was removed from the framework (spec 043).

**Target paths:** `{config_dir}/commands/{project}/workflows/`, `{config_dir}/commands/{project}/skills/`, `workflows/registry.json`, `framework/workflows/`, `framework/skills/`

Spec 043 removed the workflows feature: `/ductus` no longer offers or scaffolds tech-stack workflow commands, and nothing reads the `[workflows]` config section. This migration cleans up what earlier `/ductus` runs scaffolded. It subsumes the retired `skills-to-workflows` and `workflow-filename-rename` migrations (both archived above), so it also covers adopters carrying pre-rename debris.

Every deletion below honors `.ductus/config.toml` `[pinned] files` (path comparison after placeholder resolution): a pinned path is left alone and reported as `pinned (kept): {path}` in the post-scaffolding summary. Adopters who customized a scaffolded file and want to keep it can pin its destination path.

1. **Idempotency check.** If none of the targets below exist — no `{config_dir}/commands/{project}/workflows/` directory, no `{config_dir}/commands/{project}/skills/` directory, no `workflows/registry.json` at the project root, and no `[workflows]` section in the active config file — exit silently.

2. **Exact-set workflow-file deletion.** In `{config_dir}/commands/{project}/workflows/`, delete each unpinned file whose name appears in this exact set (13 current template names plus the 9 legacy `{category}-{language}-{tool}.md` names):

   - `black.md`, `eslint.md`, `gofmt.md`, `golangci-lint.md`, `gotest.md`, `prettier.md`, `pytest.md`, `rails-migrate.md`, `rspec.md`, `rubocop.md`, `ruff.md`, `rufo.md`, `vitest.md`
   - `format-go-gofmt.md`, `format-python-black.md`, `format-typescript-prettier.md`, `lint-go-golangci-lint.md`, `lint-python-ruff.md`, `lint-typescript-eslint.md`, `test-go-gotest.md`, `test-python-pytest.md`, `test-typescript-vitest.md`

   The check is by exact filename match; adopter-authored files (e.g., `pytest-fast.md`) are never affected because they aren't in the set. Report each deletion as `removed (workflow): {filename}`. After the pass, remove the `workflows/` directory itself only if it is empty — a directory kept alive by custom files is left in place.

3. **Legacy `skills/` removal.** If `{config_dir}/commands/{project}/skills/` exists and is not pinned, recursively delete it and report `removed (legacy skills/ directory): {config_dir}/commands/{project}/skills/`. Any remaining `skills/` tree predates the 0.2.0 rename and is necessarily stale.

4. **Synced registry removal.** If `workflows/registry.json` exists at the project root and is not pinned, delete it and report `removed (workflow registry): workflows/registry.json`. Remove the root `workflows/` directory if that leaves it empty.

5. **`[workflows]` config-section removal.** In the **active config file** (`.ductus/config.toml` when it exists, else the legacy root `.govern.toml` — the standard write policy), if a `[workflows]` section exists, remove it: the `[workflows]` header line, every key under it (`declined_categories`), and the comment lines attached directly above the header or its keys, normalizing to a single blank line between the surrounding sections. Preserve every other table, comment, and formatting byte-for-byte. Report `removed (config): [workflows] section`. If no `[workflows]` section exists, skip silently.

6. **Summary.** The lines reported above appear in the post-scaffolding summary. The migration runs once per adopter (tracked via `[migrations].last_applied`); every step is a no-op when its target is absent, so a re-run is silent.
