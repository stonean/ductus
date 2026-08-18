#!/usr/bin/env bash
# scripts/audit/sweep-target-manifest-parity.sh — Family 23 of /audit.
#
# The rename sweep required by §drift-prevention greps a fixed set of
# live-artifact paths. That set is enumerated in AGENTS.md between
# `<!-- audit:sweep-targets:begin -->` and `<!-- audit:sweep-targets:end -->`,
# and the constitution's §spec-lifecycle case (a) resolves the scope of a
# mechanical edit through it. A list naming a relocated directory sends the
# grep somewhere clean: the sweep misses the files that moved and exits 0.
#
# Not hypothetical. 042 moved the generators from `scripts/` to
# `.ductus/scripts/`; 049's sweep grepped the list, which still said
# `scripts/`; `config_path_of` in the shipped `.ductus/scripts/lib/specs-root.sh`
# kept resolving a legacy config path with no `.ductus/` tier, so a converged
# adopter fell through to the default spec root and derived nothing. Silent,
# exit 0, and invisible to every other family here.
#
# This family holds the enumeration against the **Shared Files** manifest in
# framework/bootstrap/ductus.md: every source path the framework ships must be
# covered by some entry in the list.
#
# DIRECTION. The check runs manifest → list only. It proves no shipped file
# goes unswept; it cannot prove the list is complete, because the list
# legitimately covers paths the manifest never mentions (`runtime/`,
# `.github/`, `docs/`). The scope line on stderr names the direction that ran,
# so a clean exit is never read as having verified both.
#
# SCOPE. Source paths are taken from the tables under `## Shared Files` only.
# The prose entries beside them (AGENTS.md, CLAUDE.md, .gitignore) and the
# `## Per-Agent Scaffolding` command table all name sources under `framework/`,
# which any list covering the framework tree already covers; they are counted
# and reported rather than silently dropped.
#
# A degenerate extraction — no list entries, or no manifest rows — is a
# finding, never a pass. See AGENTS.md §Design Principles: a check that cannot
# run must not be indistinguishable from one that did.
#
# Bash 3.2 compatible (macOS system bash); POSIX sed/grep only, no awk
# extensions (the portability trap that made Family 7 dead on macOS).

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family sweep-target

LIST_FILE="AGENTS.md"
MANIFEST_FILE="framework/bootstrap/ductus.md"
BEGIN_MARK="audit:sweep-targets:begin"
END_MARK="audit:sweep-targets:end"

# --- Extract the live-artifact enumeration -------------------------------
#
# The markers may sit on one line (they do today — the entry is a single long
# bullet) or span several. The one-line case must be handled separately: a sed
# range whose start and end regexes match the *same* line does not close there,
# it resumes hunting for the end pattern on the next line and runs to EOF. That
# failure is silent and generous — it yields a superset of the list, so every
# manifest path looks covered — which is why the entry count is reported.
# Both markers must be present. A lone begin marker would leave the range
# below unterminated — the same silent over-collection, reached a different
# way — so an unbalanced pair is reported rather than extracted from.
if ! grep -q -- "$BEGIN_MARK" "$LIST_FILE" 2>/dev/null \
  || ! grep -q -- "$END_MARK" "$LIST_FILE" 2>/dev/null; then
  emit "$LIST_FILE" \
    "live-artifact enumeration markers are missing or unbalanced — $BEGIN_MARK and $END_MARK must both be present" \
    "restore both delimiting comments around the enumeration in $LIST_FILE §Workflow's dead-references entry"
  exit "$drift"
fi

if grep -q -- "$BEGIN_MARK.*$END_MARK" "$LIST_FILE"; then
  raw="$(grep -- "$BEGIN_MARK.*$END_MARK" "$LIST_FILE")"
else
  raw="$(sed -n "/$BEGIN_MARK/,/$END_MARK/p" "$LIST_FILE")"
fi
segment="$(printf '%s\n' "$raw" \
  | sed -e "s/.*<!-- $BEGIN_MARK -->//" -e "s/<!-- $END_MARK -->.*//")"

entries="$(printf '%s\n' "$segment" | grep -oE '`[^`]+`' | tr -d '`' | sort -u)"
entry_count="$(printf '%s\n' "$entries" | grep -c '[^[:space:]]')"

if [ "$entry_count" -eq 0 ]; then
  emit "$LIST_FILE" \
    "live-artifact enumeration is empty or its $BEGIN_MARK / $END_MARK markers are missing — nothing to check the manifest against" \
    "restore the enumeration and its delimiting comments in $LIST_FILE §Workflow's dead-references entry"
  exit "$drift"
fi

# --- Extract the Shared Files manifest source paths -----------------------
#
# `## Shared Files` through the next `## ` heading. `### ` subheadings do not
# match `^## ` (third character is `#`, not a space), so the two strategy
# tables are both inside the range.
manifest_region="$(sed -n '/^## Shared Files$/,/^## /p' "$MANIFEST_FILE" 2>/dev/null)"
sources="$(printf '%s\n' "$manifest_region" \
  | grep -E '^\| `' \
  | sed -E 's/^\| `([^`]+)`.*/\1/' \
  | sort -u)"
source_count="$(printf '%s\n' "$sources" | grep -c '[^[:space:]]')"

if [ "$source_count" -eq 0 ]; then
  emit "$MANIFEST_FILE" \
    "no source paths extracted from the ## Shared Files manifest — the section or its tables were renamed or restructured" \
    "realign the extraction in scripts/audit/sweep-target-manifest-parity.sh with the manifest's current shape"
  exit "$drift"
fi

# --- Coverage -------------------------------------------------------------
#
# An entry ending in `/` covers everything beneath it; an entry containing a
# glob covers everything under its literal prefix; anything else must match
# exactly. Prefix matching is deliberate — requiring one list row per shipped
# file would fire on every manifest row and make the family noise.
is_covered() {
  local covered_path="$1" entry prefix
  while IFS= read -r entry; do
    [ -z "$entry" ] && continue
    case "$entry" in
      *'*'*)
        prefix="${entry%%\**}"
        case "$covered_path" in "$prefix"*) return 0 ;; esac
        ;;
      */)
        case "$covered_path" in "$entry"*) return 0 ;; esac
        ;;
      *)
        [ "$covered_path" = "$entry" ] && return 0
        ;;
    esac
  done <<< "$entries"
  return 1
}

templated=0
examined=0
while IFS= read -r src; do
  [ -z "$src" ] && continue
  # A templated source (`framework/bootstrap/configure/{key}.md`) names no
  # single file. Counted and reported below rather than dropped.
  case "$src" in
    *'{'*)
      templated=$((templated + 1))
      continue
      ;;
  esac
  examined=$((examined + 1))
  if ! is_covered "$src"; then
    emit "$LIST_FILE" \
      "Shared Files manifest ships \`$src\`, which no live-artifact entry covers — a rename sweep would skip it" \
      "add the covering path to the enumeration between the $BEGIN_MARK / $END_MARK markers in $LIST_FILE"
  fi
done <<< "$sources"

printf 'sweep-target: %d enumeration entr%s, %d manifest source path%s examined' \
  "$entry_count" "$([ "$entry_count" -eq 1 ] && echo y || echo ies)" \
  "$examined" "$([ "$examined" -eq 1 ] && echo '' || echo s)" >&2
if [ "$templated" -gt 0 ]; then
  printf ', %d templated path%s skipped' \
    "$templated" "$([ "$templated" -eq 1 ] && echo '' || echo s)" >&2
fi
printf '; direction verified: manifest -> list (list completeness not checked)\n' >&2

exit "$drift"
