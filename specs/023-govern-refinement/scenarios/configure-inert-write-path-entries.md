---
section: "5–6. /configure canonical allow-set"
---

# Configure-inert-write-path-entries

## Context

`framework/bootstrap/configure/claude.md` §5 lists four canonical allow entries for the ductus state files:

```text
Edit(.ductus/session.toml)    Write(.ductus/session.toml)
Edit(.ductus/config.toml)     Write(.ductus/config.toml)
```

Claude Code's permission matcher does not consult `Write(path)` rules at all — file permission checks match only `Edit(path)` entries, and an `Edit(path)` rule already covers every file-editing tool, `Write` included. The two `Write(...)` entries are therefore **inert**: they grant nothing the adjacent `Edit(...)` entries do not already grant, and the host warns about each at session start:

```text
Permission allow rule (.claude/settings.local.json): Write(.ductus/session.toml) is not
matched by file permission checks — only Edit(path) rules are. Use
Edit(.ductus/session.toml) instead (Edit rules cover all file-editing tools).
```

This is the same defect class the sibling scenario `configure-permission-pattern-safety` addressed — a canonical allow entry whose *shape* is wrong, surfaced by the host's own startup linter rather than by `/ductus:audit` — but the opposite failure mode. There the pattern granted too much; here it grants nothing. And unlike the seven `git -C` entries, which spec 023 task 21 already removed, these two are **still canonical**, so every fresh `/ductus:configure` run reinstalls them into a new adopter.

The bare `Edit` and `Write` tool-level entries at the top of §5 are unaffected: they name tools, not paths, and the warning is specific to the path-scoped form.

`claude.md` is the only host affected — `auggie.md`, `antigravity.md`, and `opencode.md` express file permissions in their own grammars and carry no `Write(path)` entries.

## Behavior

- `Write(.ductus/session.toml)` and `Write(.ductus/config.toml)` are removed from the canonical `permissions.allow` set in `framework/bootstrap/configure/claude.md` §5. The `Edit(...)` entries for both paths stay — they are what actually grants the access, for every file-editing tool.
- The removal leaves a rationale comment alongside the retained `Edit(...)` entries, so a future editor adding a path-scoped permission reaches for `Edit(...)` rather than re-deriving the rule from a startup warning — matching how the `git -C` removal documented itself.
- The canonical allow-set gains a second shape invariant: **a path-scoped allow entry uses `Edit(path)`, never `Write(path)`.** It is stated alongside the wildcard-position invariant, since both are "this entry's shape is wrong" rather than "this entry is unnecessary".
- The bare tool-level `Edit` and `Write` entries are explicitly retained and documented as distinct from the path-scoped form, so this removal is not read as license to drop `Write` as well.
- `.claude/commands/ductus/configure.md` follows from the source rewrite via the normal command generator (the pre-commit hook on `framework/bootstrap/configure/claude.md`, spec 017 row 11); no separate edit.
- Whether the invariant is enforced by extending Family 29 (`scripts/audit/permission-wildcard-position.sh`) or by a new audit family is a plan-phase decision. The two invariants share a subject (the shape of entries in a host's canonical allow-set) and the same two-host applicability, but Family 29's name and its deny-set exclusion reasoning are specific to wildcard position — a `Write(path)` entry on the deny side is inert rather than dangerous, so the exclusion does not carry across unchanged.
- `/ductus:configure` does **not** strip the two entries from an adopter's existing `settings.local.json`. That is the migration's job, specified by `retired-permission-entry-cleanup` on spec 027. This scenario only stops the framework from shipping them.

## Edge Cases

- An adopter who ran `/ductus:configure` before this change keeps both entries until the 027 migration runs; the host's startup warning is what surfaces them meanwhile.
- Removing the bare `Write` tool entry would revoke a real tool grant, not a path grant — the two forms must not be swept together.
- A host whose permission grammar has no path-scoped file form cannot express the defect, and is skipped by whatever audit check enforces the invariant — the same per-host applicability rule the wildcard-position check already applies.
- An adopter who pinned `configure/claude.md` in `.ductus/config.toml` keeps their pinned copy, inert entries included; the pin contract already accepts that trade.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
