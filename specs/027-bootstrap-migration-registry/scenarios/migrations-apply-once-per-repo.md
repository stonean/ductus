---
section: "Adopter State"
---

# Migrations-apply-once-per-repo

## Context

A migration applies **once per repository**, not once per contributor, and the contract never says so.

`[migrations].last_applied` lives in the **committed** `.ductus/config.toml`. The first contributor to run `/ductus` applies the pending entries and commits the advanced marker; every teammate who pulls afterwards reads it, sees nothing pending, and skips. That is correct and intended for repo-shared state — the edit genuinely happened once, for everyone.

For per-contributor state it means the migration reaches exactly one person. Two active entries already touch such state and both are fine, which is why the limit went unnoticed:

- `session-file-consolidate` targets `{config_dir}/{project}-session.json`, and `ductus-rename` moves `.govern/` (which held the gitignored session file). A teammate who skips either keeps a stale session file — and their next `/{project}:target` writes to the active path anyway, under the newest-tier rule. The procedure also exits silently when the legacy file is absent, so a contributor who never used `/target` has nothing to do.

Both **self-heal**: the skipped work is redone by ordinary use, or was never needed. The registry is safe for them.

It is not safe when the migration is the *only* thing that would ever repair the defect. Spec 023 needed to retire nine formerly-canonical entries from `{cli-config-dir}/settings.local.json` — gitignored, per-contributor, and rewritten by no other automatic path. Routed through the registry: Alice runs `/ductus`, her file is cleaned, the marker advances, she commits it; Bob pulls, runs `/ductus`, and the loop skips the entry. Bob keeps all nine — including seven that approve arbitrary execution with no prompt — permanently, and no future `/ductus` run can fix it, because the marker says the work is done. The migration reports success on a repo where most contributors were never touched.

A second constraint compounded it for that target: `/ductus` never writes the full permission set at all. It seeds only the bootstrap entries and directs the operator to run `/{project}:configure` afterwards (`ductus.md` §Does Not Do, §Post-Scaffolding Output). The retirement was therefore homed on `/ductus:configure` (spec 023, `configure-retires-formerly-canonical-entries`), which runs per-contributor, owns the file, and is idempotent — so it needs no marker at all. What remains here is the limit that sent it there.

## Behavior

- 027's contract states the scope of a migration plainly: **the registry applies an entry once per repository.** A migration whose target is per-contributor reaches only the contributor who runs it first.
- The rule that follows is about remedy, not about paths: **a migration must not be the sole remedy for a defect in per-contributor state.** Touching per-contributor state is allowed when a skipped contributor self-heals through ordinary use — the two existing entries qualify and are named as the precedent — and disallowed when skipping leaves a defect nothing else will repair.
- A cleanup that must reach every contributor is directed instead to the command that owns the per-contributor file, which runs per-contributor and can be idempotent. The permission retirement is named as the worked example, along with why `/ductus` could not host it even setting the marker aside.
- The rule is recorded where a migration author will meet it — `framework/migrations.toml`'s header comment, beside the existing note on what `target_paths` drives — not only in this spec.
- `target_paths` needs **no** change. It already names paths a migration *acts on* rather than paths that go away: `gitignore-marker-rename` targets `.gitignore` and edits it in place. Family 10's no-stale-target-paths check constrains only `framework/`-prefixed entries, treating adopter-relative paths as unverifiable from this repo. An edited-not-removed adopter path was always inside the contract, so the question raised when the retirement was first routed here is resolved by existing precedent rather than by a new field.

## Edge Cases

- A migration that only *reads* per-contributor state to decide a repo-shared edit is unaffected — the marker is correct, because the repo-shared edit does happen once.
- "Per-contributor" means what the framework's own layout guarantees (gitignored by the shipped `.gitignore`), not whatever an individual adopter has chosen to ignore locally.
- `{config_dir}/commands/` is **not** per-contributor despite living under `.claude/`: the shipped `.gitignore` excepts it (`!.claude/commands/`) so slash commands stay tracked. `workflows-sunset`, which targets that directory, is a repo-shared migration.
- An adopter working alone sees no difference — the marker and the per-contributor file belong to the same person — which is exactly why this class of defect survives testing and reaches teams only.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
