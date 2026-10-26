#!/usr/bin/env bash
# Verify every reference to a runtime tool in framework/commands/*.md
# is paired with a graceful-fallback marker within 20 lines.
#
# Tool names are read from framework/runtime-tools.txt (one per non-blank,
# non-comment line; case-sensitive exact match). Fallback markers are
# any case-insensitive occurrence of: Otherwise, Fallback, If unavailable,
# markdown-only path. The proximity scan trades false-positive risk for a
# fully derived check — no author-supplied markers required.
#
# A command's markdown-only reference — the fallback itself — lives in
# framework/reference/<name>.md (spec 071), outside this scan's subject, so
# a tool named there needs no paired marker and is never examined.
#
# A command that carries the constitution's host-integration pointer
# (a reference to §runtime-host-integration) is exempt file-wide: the
# pointer declares the markdown-only fallback contract once for the whole
# command instead of restating it per Invoke step (constitution
# §runtime-host-integration; spec 022 scenario review-runtime-acceleration
# #3/#4 prose tightening). Legacy commands without the pointer keep the
# per-reference proximity check below.
#
# Source of truth: framework/constitution.md §runtime-boundary
# Consumed by: .github/workflows/framework-checks.yml

set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

WINDOW=20
MANIFEST="$ROOT/framework/runtime-tools.txt"

for arg in "$@"; do
  case "$arg" in
    -h|--help)
      sed -n '2,24p' "$0" | sed 's/^# \{0,1\}//'
      echo
      echo "Usage: $(basename "$0")"
      echo "  Exits 0 when every runtime-tool reference has a fallback within $WINDOW lines."
      echo "  Exits 1 when any reference lacks a fallback (errors printed to stdout)."
      exit 0
      ;;
    *) echo "Unknown argument: $arg" >&2; exit 2 ;;
  esac
done

if [ ! -f "$MANIFEST" ]; then
  echo "Manifest not found: $MANIFEST" >&2
  exit 2
fi

# Load tool names: skip blank lines and lines starting with '#'.
# Strip a full run of leading/trailing whitespace (spaces or tabs) so a
# stray indent in the manifest can't slip through with embedded
# whitespace that would silently fail to match in prose.
tools=()
while IFS= read -r raw; do
  line="${raw%%#*}"
  line="${line#"${line%%[![:space:]]*}"}"
  line="${line%"${line##*[![:space:]]}"}"
  [ -z "$line" ] && continue
  tools+=("$line")
done < "$MANIFEST"

if [ "${#tools[@]}" -eq 0 ]; then
  exit 0
fi

shopt -s nullglob
errors=0
for cmd in "$ROOT"/framework/commands/*.md; do
  # File-wide exemption: a command that points to the constitution's
  # host-integration contract has declared the markdown-only fallback once
  # for every Invoke step, so no per-reference proximity marker is required.
  if grep -q 'runtime-host-integration' "$cmd"; then
    continue
  fi
  for tool in "${tools[@]}"; do
    while IFS=: read -r lineno _; do
      [ -z "$lineno" ] && continue
      end=$((lineno + WINDOW))
      window="$(sed -n "${lineno},${end}p" "$cmd")"
      if ! printf '%s\n' "$window" | grep -qiE 'Otherwise|Fallback|If unavailable|markdown-only path'; then
        echo "${cmd#"$ROOT"/}:${lineno}: missing fallback for tool '${tool}'"
        errors=$((errors + 1))
      fi
    done < <(grep -nF -- "$tool" "$cmd" || true)
  done
done

if [ "$errors" -gt 0 ]; then
  exit 1
fi
exit 0
