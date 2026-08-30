---
section: "5–6. /configure canonical allow-set"
---

# Configure-retires-formerly-canonical-entries

## Context

Fixing the canonical set stops new adopters getting a bad entry. It does nothing for the adopters who already have one.

Two removals are in flight. Task 21 dropped seven `Bash(git -C * <sub> *)` allow entries because the leading wildcard spans inserted options and both `-c` and `--exec-path` run arbitrary commands. The sibling scenario `configure-inert-write-path-entries` drops two `Write(.ductus/*.toml)` entries that grant nothing. In both cases the source is now correct — and `merge-permissions` installs and dedups but, by design, never removes a non-canonical entry an adopter owns. So an adopter configured before either change keeps the entries indefinitely, and Claude Code warns about all nine at every session start.

`configure-permission-pattern-safety` named that outcome and accepted it: *"`/ductus:configure` does not retroactively strip an unsafe pattern already present in an adopter's `settings.local.json` … Adopters configured before this change keep the seven rules until they remove them, and their host's startup warning is what surfaces it."* **That decision is superseded here.** Seven of the nine approve arbitrary execution with no prompt; leaving them installed and putting the repair on the adopter leaves a security hole open until someone reads a warning and acts on it.

The obvious owner is the migration registry — `/ductus` §Pre-run Migrations is literally "adopter-side cleanup for conventions removed since the adopter's last `/ductus` run." It does not work here, for two independent reasons:

- **The marker is repo-shared; the target is not.** `[migrations].last_applied` lives in the committed `.ductus/config.toml`, while `{cli-config-dir}/settings.local.json` is gitignored and per-contributor. The first teammate to run `/ductus` would clean their own file and commit a marker that makes every other teammate skip the entry — permanently, since no later run can undo a marker that says the work is done. Recorded as spec 027's `migrations-apply-once-per-repo`.
- **`/ductus` does not own this file.** It seeds only the bootstrap permission entries and directs the operator to run `/{project}:configure` afterwards (`ductus.md` §Does Not Do, §Post-Scaffolding Output).

`/ductus:configure` has neither problem. It is the sole owner of the file, it runs per-contributor, and it is idempotent — so retirement needs no marker at all, and happens on the next run for each person who runs it.

## Behavior

- `/ductus:configure` gains a fourth numbered section: **retired `permissions.allow` entries**, listing the nine entries the framework once shipped and now removes, each grouped under the reason it was retired and the version that retired it.
- The list is passed to `merge-permissions` as `revoke`. The primitive removes exact matches only, so an entry an adopter authored is never touched however closely it resembles a retired one — shape-matching is deliberately not offered.
- Retirement is **allow-side only, enforced by the primitive's shape** rather than by prose: `merge-permissions` has no deny-side counterpart. An over-broad *denial* refuses more rather than approving more, so the `Bash(git -C * …)` patterns in the canonical deny set are correct, and an argument that could sweep both arrays would invite narrowing them into holes.
- The revoke pass runs **before** dedup and canonical-presence, so every copy of a retired entry is removed and none is re-added, and a doubled retired entry is attributed wholly to retirement rather than split across two counters.
- An entry present in both the canonical set and the retired list is **rejected** (`ConflictingRevoke`), not resolved by pass order: the two passes would fight, the merge would never reach a fixed point, and the `unchanged` short-circuit that preserves mtime could never fire.
- `/ductus:audit` Family 32 checks the same disjointness at maintainer time, so the editing slip fails in this repo rather than in every adopter's `/ductus:configure` run. Retiring an entry means deleting it from the canonical set in the same edit.
- The command reports what it retired, naming each removed entry, so the adopter sees the change to their permission file rather than finding it by diff.
- The markdown-only path performs the same splice by exact string match, before its own presence and dedup passes — one contract, two paths.

## Edge Cases

- **The adopter must run `/ductus:configure` for the cleanup to land.** `/ductus` alone does not do it. The post-scaffolding output already names the command as step 1 of next steps, and an adopter who never runs it keeps the entries — the same exposure as any permission change the command owns.
- An adopter who pinned `configure/claude.md` in `.ductus/config.toml` keeps their pinned copy, retired entries and all; the pin contract already accepts that trade.
- A retired entry an adopter has deliberately re-added is removed again on the next run. That is the intended behavior for an arbitrary-execution hole, and the pin is the documented opt-out.
- The list is append-only across releases. An entry is never removed from it once added, because an adopter arriving from any older version must still be cleaned.
- A hand-edited `settings.local.json` that is invalid JSON is reported and left alone, never silently rewritten — `merge-permissions` refuses on parse error before writing anything.
- A non-Claude host has no entry in this list to retire: the seven `git -C` patterns and the two `Write(path)` entries only ever shipped in `configure/claude.md`, and the other three hosts' grammars cannot express them.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
