#!/usr/bin/env bash
# Verify that every framework/commands/*.md file naming a runtime tool
# carries the constitution's host-integration pointer: a reference to
# §runtime-host-integration, the one line that says how the command's Invoke
# steps reach the runtime and what the host walks before its server is
# registered. The pointer replaced the per-step markdown-only fallback this
# lint once paired with each tool reference (spec 048, AC17).
#
# Tool names are read from framework/runtime-tools.txt: each non-blank,
# non-comment line, and each name on a `# cli-only:` line. A command names a
# tool when the name occurs in it (case-sensitive substring, so a longer word
# containing one counts too, which errs toward requiring the pointer). A
# command naming none, such as help.md, needs no pointer.
#
# Every run prints what it examined, so a clean result says how much it
# judged; a manifest with no tool names, or no command files, is an error
# rather than a pass, since either would judge nothing.
#
# Source of truth: framework/constitution.md §runtime-host-integration
# Consumed by: .github/workflows/framework-checks.yml, scripts/audit/check-zero.sh
# Tested by: scripts/tests/test-lint-tool-coverage.sh

set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

MANIFEST="$ROOT/framework/runtime-tools.txt"

for arg in "$@"; do
  case "$arg" in
    -h|--help)
      sed -n '2,21p' "$0" | sed 's/^# \{0,1\}//'
      echo
      echo "Usage: $(basename "$0")"
      echo "  Exits 0 when every command naming a runtime tool carries the pointer."
      echo "  Exits 1 when one does not (each printed to stdout)."
      echo "  Exits 2 when the manifest is missing or names no tool, or no command file exists."
      exit 0
      ;;
    *) echo "Unknown argument: $arg" >&2; exit 2 ;;
  esac
done

if [ ! -f "$MANIFEST" ]; then
  echo "Manifest not found: $MANIFEST" >&2
  exit 2
fi

# Load tool names: a `# cli-only:` line's name, and every other line up to its
# first '#'. Strip a full run of leading/trailing whitespace (spaces or tabs)
# so a stray indent in the manifest can't slip through with embedded
# whitespace that would silently fail to match in prose.
tools=()
while IFS= read -r raw; do
  case "$raw" in
    '# cli-only:'*) line="${raw#'# cli-only:'}" ;;
    *) line="${raw%%#*}" ;;
  esac
  line="${line#"${line%%[![:space:]]*}"}"
  line="${line%"${line##*[![:space:]]}"}"
  [ -z "$line" ] && continue
  tools+=("$line")
done < "$MANIFEST"

if [ "${#tools[@]}" -eq 0 ]; then
  echo "No tool names in ${MANIFEST#"$ROOT"/}: nothing to examine" >&2
  exit 2
fi

shopt -s nullglob
examined=0
naming=0
errors=0
for cmd in "$ROOT"/framework/commands/*.md; do
  examined=$((examined + 1))
  named=""
  for tool in "${tools[@]}"; do
    if grep -qF -- "$tool" "$cmd"; then
      named="$tool"
      break
    fi
  done
  if [ -z "$named" ]; then
    continue
  fi
  naming=$((naming + 1))
  if ! grep -q 'runtime-host-integration' "$cmd"; then
    echo "${cmd#"$ROOT"/}: names runtime tool '${named}' but carries no §runtime-host-integration pointer"
    errors=$((errors + 1))
  fi
done

if [ "$examined" -eq 0 ]; then
  echo "No framework/commands/*.md file found: nothing to examine" >&2
  exit 2
fi

echo "lint-tool-coverage: ${examined} command file(s) examined, ${naming} naming a runtime tool, $((naming - errors)) of them carrying the pointer"
if [ "$errors" -gt 0 ]; then
  exit 1
fi
exit 0
