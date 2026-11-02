#!/usr/bin/env bash
# Verify rule-ID grammar and per-file uniqueness in framework/rules/*.md.
#
# Per the rule schema, every level-3 heading in a rule file is a rule ID and
# carries nothing but the ID. Each MUST match the grammar
# `{SURFACE}-{CATEGORY}-{NNN}`:
#   - SURFACE   one of BE, FE, CFG, QUAL, SEC (registered surfaces)
#   - CATEGORY  [A-Z][A-Z0-9]*  (uppercase alphanumeric; first char a letter)
#   - NNN       a 3- or 4-digit zero-padded sequence number
# IDs MUST be unique within a file.
#
# This catches the failure mode where a category abbreviation drifts from the
# schema grammar that the runtime `check-rule-ids` harvester relies on
# (e.g. a digit-bearing category the harvester regex does not match).
#
# Source of truth: specs/008-security-rules/data-model.md (BE/FE surfaces),
#                  specs/017-derive-dont-ask/data-model.md (CFG surface),
#                  specs/036-quality-cross-rules/data-model.md (QUAL surface),
#                  specs/077-security-cross-rules/data-model.md (SEC surface),
#                  framework/constitution.md §rules
# Consumed by: .github/workflows/framework-checks.yml

set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

for arg in "$@"; do
  case "$arg" in
    -h|--help)
      sed -n '2,20p' "$0" | sed 's/^# \{0,1\}//'
      echo
      echo "Usage: $(basename "$0")"
      echo "  Exits 0 when every rule-ID heading is well-formed and unique per file."
      echo "  Exits 1 when any heading fails (errors printed to stdout),"
      echo "  and when no rule file is found at all — examining nothing is not a pass."
      echo "  The examined file and heading counts are printed to stderr on every run."
      exit 0
      ;;
    *) echo "Unknown argument: $arg" >&2; exit 2 ;;
  esac
done

shopt -s nullglob

# Registered surfaces. Extend deliberately when a new rule-introducing spec
# registers a surface (mirrors the intent of the closed-suffix filename policy).
id_re='^(BE|FE|CFG|QUAL|SEC)-[A-Z][A-Z0-9]*-[0-9]{3,4}$'

errors=0
checked=0
files=("$ROOT"/framework/rules/*.md)

# A lint that examined nothing must not exit like one that examined everything
# (constitution §design-principles; QUAL-CLAIM-001). With `nullglob` a missing or
# relocated framework/rules/ yields an empty array, and on bash >= 4.4 — which is
# what ubuntu-latest runs — expanding it under `set -u` is not an error, so the
# loop below would simply not run and this script would exit 0 having read no
# rule file at all. Fail loudly instead, and name the subject rather than letting
# bash 3.2's opaque `files[@]: unbound variable` stand in for a diagnostic.
if [ "${#files[@]}" -eq 0 ]; then
  echo "lint-rule-ids: no rule files found under framework/rules/ — examined nothing, so this run is not evidence" >&2
  exit 1
fi

for f in "${files[@]}"; do
  [ -f "$f" ] || continue
  rel="${f#"$ROOT"/}"
  seen=""

  # Level-3 headings only: the schema reserves `### ` for rule IDs.
  while IFS= read -r line; do
    id="${line#"### "}"
    id="${id%%[[:space:]]*}"   # defensive: drop any trailing content

    checked=$((checked + 1))

    if [[ ! "$id" =~ $id_re ]]; then
      echo "$rel: malformed rule ID heading: '### $id' (expected {BE|FE|CFG|QUAL|SEC}-{CATEGORY}-{NNN})"
      errors=$((errors + 1))
      continue
    fi

    case " $seen " in
      *" $id "*)
        echo "$rel: duplicate rule ID: $id"
        errors=$((errors + 1))
        ;;
      *) seen="$seen $id" ;;
    esac
  done < <(grep -E '^### ' "$f" || true)
done

# The examined counts are the guard: they are what makes a clean exit 0 a claim
# about a subject rather than a bare success string.
echo "lint-rule-ids: examined ${#files[@]} rule file(s), $checked rule ID heading(s)" >&2

if [ "$errors" -gt 0 ]; then
  exit 1
fi
exit 0
