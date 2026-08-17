#!/usr/bin/env bash
# .ductus/scripts/lib/specs-root.sh — shared spec-tree helpers for the generators.
#
# Sourced by gen-spec-deps.sh and gen-cross-service-refs.sh, which run in both
# ductus's own pre-commit and adopter pre-commit hooks. One definition rather
# than a copy per generator (spec 040 review — no drift). Shipped to adopters
# via the Shared Files manifest in framework/bootstrap/ductus.md alongside the
# generators; sourced by script-relative path so it resolves regardless of cwd
# or a --root override (which retargets $ROOT but not the script location).
#
# The sourcing script MUST define $ROOT (its repo-root variable) before calling
# any of these, and $SPECS_ROOT (resolve_specs_root's result) before calling
# list_specs / staged_specs; both generators set them up top.

# Spec-root directory name from a given ductus config file's [paths]
# specs-root,
# defaulting to "specs" (spec 040) when the file is absent, the key is missing,
# or the value is outside the [A-Za-z0-9_-] charset (path separators, "..",
# ".", or any regex metacharacter). That conservative charset matches the
# runtime's validate_specs_root, so the name is safe both as a path component
# and when interpolated into the grep/awk regexes the generators build from it.
#
# Takes the config-file path as an argument so callers can resolve THIS repo's
# root (resolve_specs_root, below) or a *referenced* service's own root from its
# local checkout (gen-cross-service-refs.sh's checkout-reachable matcher tier —
# scenario referenced-service-spec-root).
specs_root_of() {
  local toml="$1" name=""
  if [ -f "$toml" ]; then
    name="$(awk '
      /^\[/ { in_paths = ($0 ~ /^\[paths\][[:space:]]*$/); next }
      in_paths && /^[[:space:]]*specs-root[[:space:]]*=/ {
        line = $0
        sub(/^[^=]*=[[:space:]]*/, "", line)
        if (match(line, /"[^"]*"/)) { print substr(line, RSTART + 1, RLENGTH - 2) }
        else { sub(/[[:space:]]*(#.*)?$/, "", line); print line }
        exit
      }
    ' "$toml")"
  fi
  case "$name" in
    "" | *[!A-Za-z0-9_-]*) name="specs" ;;
  esac
  printf '%s' "$name"
}

# Active ductus config file for a repo/checkout root: `.ductus/config.toml`
# when it exists, else `.govern/config.toml`, else the legacy root
# `.govern.toml` (specs 042/049 — newest tier wins on a split layout; when
# none exists the legacy path is returned and specs_root_of falls through to
# the default on the absent file). All three tiers are required: omitting the
# `.ductus/` one resolves a *converged* adopter to a path that does not exist,
# which specs_root_of answers with the default `specs` — so a non-default
# [paths] specs-root enumerates the wrong tree, derives nothing, and reports
# success. Mirrors the runtime's config-path ladder; keep them in step.
config_path_of() {
  local base="$1"
  if [ -f "$base/.ductus/config.toml" ]; then
    printf '%s' "$base/.ductus/config.toml"
  elif [ -f "$base/.govern/config.toml" ]; then
    printf '%s' "$base/.govern/config.toml"
  else
    printf '%s' "$base/.govern.toml"
  fi
}

# Spec-root directory name for THIS repo (its active config file).
resolve_specs_root() {
  specs_root_of "$(config_path_of "$ROOT")"
}

# Feature-spec files to process, scoped to the git index (tracked + staged)
# rather than a worktree glob. Untracked, in-progress drafts — e.g. a /specify
# spec the author has not `git add`ed yet — are intentionally excluded so they
# are never rewritten, never enter a generator's graph, and never block an
# unrelated commit (spec 017 / tracked-specs-not-worktree). Falls back to a
# worktree glob only outside a git repo, where there is no index.
list_specs() {
  if git -C "$ROOT" rev-parse --git-dir >/dev/null 2>&1; then
    git -C "$ROOT" ls-files -- "$SPECS_ROOT" \
      | { grep -E "^$SPECS_ROOT/[0-9][0-9][0-9]-[^/]+/(spec|spec-and-plan)\.md$" || true; } \
      | while IFS= read -r rel; do printf '%s/%s\n' "$ROOT" "$rel"; done
  else
    local f
    for f in "$ROOT"/"$SPECS_ROOT"/[0-9][0-9][0-9]-*/spec.md "$ROOT"/"$SPECS_ROOT"/[0-9][0-9][0-9]-*/spec-and-plan.md; do
      [ -e "$f" ] && printf '%s\n' "$f"
    done
  fi
}

# Feature-spec files present in the worktree but NOT tracked by git — exactly
# the set list_specs excludes by design (spec 017 / tracked-specs-not-worktree).
#
# Exists so a generator can say what it did not examine. A zero rewrite count
# means "I rewrote nothing", not "everything is in sync": an untracked draft is
# never enumerated, so a bare in-sync claim asserts a property of files the
# generator cannot vouch for (QUAL-CLAIM-001, and the case an adopter lost time
# to on 2026-08-01). Reporting only — the exclusion itself is unchanged.
# Empty outside a git repo, where list_specs already falls back to a full
# worktree glob and therefore examines everything.
untracked_specs() {
  if git -C "$ROOT" rev-parse --git-dir >/dev/null 2>&1; then
    git -C "$ROOT" ls-files --others --exclude-standard -- "$SPECS_ROOT" \
      | { grep -E "^$SPECS_ROOT/[0-9][0-9][0-9]-[^/]+/(spec|spec-and-plan)\.md$" || true; } \
      | while IFS= read -r rel; do printf '%s/%s\n' "$ROOT" "$rel"; done
  fi
}

# Render the honest no-change line for a generator whose enumeration is
# list_specs-scoped: what was examined, and what was skipped.
#
# `$1` (optional, default 0) is the count of specs the generator examined and
# found drifted but deliberately did NOT rewrite — the `--staged` case, where a
# tracked-but-unstaged spec's derived field has drifted and is left alone so
# committing one spec never rewrites another. Those are neither "in sync" nor
# "not examined", and a zero rewrite count would otherwise report them as the
# former.
report_no_changes() {
  local unwritten="${1:-0}" tracked untracked line
  tracked=$(list_specs | wc -l | tr -d ' ')
  untracked=$(untracked_specs | wc -l | tr -d ' ')
  line="No changes ($((tracked - unwritten)) tracked spec(s) in sync"
  if [ "$unwritten" -gt 0 ]; then
    line="$line; $unwritten drifted spec(s) left unwritten — not staged"
  fi
  if [ "$untracked" -gt 0 ]; then
    line="$line; $untracked untracked spec(s) skipped — git add to include"
  fi
  echo "$line)"
}

# Feature-spec files staged in the git index for the pending commit — the
# --staged rewrite set (the adopter pre-commit path), so committing one spec
# never rewrites the derived frontmatter of unrelated specs. Empty outside a
# git repo. How each generator combines this with list_specs is its own
# concern (see the call sites).
staged_specs() {
  git -C "$ROOT" rev-parse --git-dir >/dev/null 2>&1 || return 0
  git -C "$ROOT" diff --cached --name-only -- "$SPECS_ROOT" \
    | { grep -E "^$SPECS_ROOT/[0-9][0-9][0-9]-[^/]+/(spec|spec-and-plan)\.md$" || true; } \
    | while IFS= read -r rel; do printf '%s/%s\n' "$ROOT" "$rel"; done
}
