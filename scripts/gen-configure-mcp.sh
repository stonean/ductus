#!/usr/bin/env bash
# Regenerate the runtime-MCP-tool permission block in
# framework/bootstrap/configure/claude.md from the canonical tool list in
# framework/runtime-tools.txt.
#
# Establishes the invariant: every tool listed in runtime-tools.txt has
# a permission entry in Claude's configure source. Adding or removing a
# tool in runtime-tools.txt flows through to it on the next commit via
# the pre-commit hook.
#
# Pi (064) is excluded by name from this generator: Pi has no
# MCP-permission shape — its tools arrive through the
# `.pi/extensions/ductus.ts` extension bridge (which owns the
# `ductus__<name>` naming), and Pi has no permission-gating settings at
# all, so there is no permission block to generate and no canonical entry
# to keep in parity. The pi exclusion is stated here rather than silently
# omitted: a future agent with a true MCP-permission shape gets its own
# configure source and generator step.
#
# Marker pair:
#   <!-- generated:mcp-allow:start -->
#   <!-- generated:mcp-allow:end -->
#
# Mapping (deterministic; no host-presence detection). The `ductus`
# server-name prefix comes from Claude's MCP registration target
# (ductus.md §MCP registration), the repo-committed `.mcp.json`. Tool names
# in the list are bare `<verb>-<noun>`.
#   <verb>-<noun>  →  Claude:  mcp__ductus__<verb>-<noun>
#
# Exits non-zero, and writes nothing to that file, if a source file's markers
# are missing, repeated, sharing a line, or out of order, since the splice
# below would write a wrong file, or silently skip it, for a file shaped that
# way.

set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TOOLS="$ROOT/framework/runtime-tools.txt"
CLAUDE_SRC="$ROOT/framework/bootstrap/configure/claude.md"

# Track every mktemp we create so early-exit paths (set -e, signals,
# splice failures) don't leak temp files into $TMPDIR.
cleanup_files=()
# `if` rather than a trailing `&&`: with the array still empty — every early
# exit above the first mktemp, including `--help` — the `&&` short-circuits to
# 1, and that becomes the EXIT trap's status and therefore the script's, so
# `--help` exited 1 and `--bogus` reported 1 instead of its documented 2. The
# `|| true` variant would fix the status by also swallowing a real `rm` failure.
cleanup() {
  if [ "${#cleanup_files[@]}" -gt 0 ]; then
    rm -f "${cleanup_files[@]}"
  fi
}
trap cleanup EXIT

dry_run=0
for arg in "$@"; do
  case "$arg" in
    --dry-run) dry_run=1 ;;
    -h|--help)
      sed -n '2,33p' "$0" | sed 's/^# \{0,1\}//'
      echo
      echo "Usage: $(basename "$0") [--dry-run]"
      echo "  --dry-run  Report what would change; exit 1 if any source needs updating."
      exit 0
      ;;
    *) echo "Unknown argument: $arg" >&2; exit 2 ;;
  esac
done

if [ ! -f "$TOOLS" ]; then
  echo "Missing source: $TOOLS" >&2
  exit 3
fi

# Build the Claude block content (3-space indent matches sibling sub-sections
# in claude.md's allow-list). Each tool name from runtime-tools.txt is a
# bare `<verb>-<noun>`; the `ductus` server-name prefix is added here.
claude_block_file="$(mktemp)"; cleanup_files+=("$claude_block_file")

tool_count=0
while IFS= read -r tool; do
  case "$tool" in
    ''|'#'*) continue ;;
  esac
  # Trim trailing whitespace.
  tool="${tool%"${tool##*[![:space:]]}"}"
  [ -z "$tool" ] && continue
  printf '   - `mcp__ductus__%s`\n' "$tool" >> "$claude_block_file"
  tool_count=$((tool_count + 1))
done < "$TOOLS"

if [ "$tool_count" -eq 0 ]; then
  echo "No tools found in $TOOLS" >&2
  exit 4
fi

# Splice a block file between markers in a target file. Fails unless each
# marker appears exactly once, the two on different lines, with the end
# after the start. A count or line number that is empty or not a number
# refuses rather than passes.
splice() {
  local file="$1"
  local block_file="$2"
  local marker n start_line end_line
  if [ ! -f "$file" ]; then
    echo "Missing source $file" >&2
    return 5
  fi
  for marker in start end; do
    n="$(grep -a -c "<!-- generated:mcp-allow:$marker -->" "$file" || true)"
    if [ "$n" = 0 ]; then
      echo "Missing marker <!-- generated:mcp-allow:$marker --> in $file" >&2
      return 5
    fi
    if [ "$n" != 1 ]; then
      echo "Marker <!-- generated:mcp-allow:$marker --> appears $n times in $file; expected once" >&2
      return 5
    fi
  done
  start_line="$(grep -a -n '<!-- generated:mcp-allow:start -->' "$file" | cut -d: -f1)"
  end_line="$(grep -a -n '<!-- generated:mcp-allow:end -->' "$file" | cut -d: -f1)"
  if [ "$start_line" = "$end_line" ]; then
    echo "Markers <!-- generated:mcp-allow:start --> and <!-- generated:mcp-allow:end --> share line $start_line in $file" >&2
    return 5
  fi
  if ! [ "$start_line" -lt "$end_line" ] 2>/dev/null; then
    echo "Marker <!-- generated:mcp-allow:end --> (line $end_line) comes before the start marker (line $start_line) in $file" >&2
    return 5
  fi
  awk -v block_file="$block_file" '
    /<!-- generated:mcp-allow:start -->/ {
      print
      while ((getline line < block_file) > 0) print line
      close(block_file)
      in_block = 1
      next
    }
    /<!-- generated:mcp-allow:end -->/ {
      in_block = 0
      print
      next
    }
    !in_block { print }
  ' "$file"
}

# Process each file. Compare against the source; rewrite or report.
# `out` is mktemp'd and registered for trap cleanup; the happy path
# either `mv`s it onto $file (consuming the temp) or leaves it to the
# trap.
process() {
  local file="$1"
  local block_file="$2"
  local out
  out="$(mktemp)"
  cleanup_files+=("$out")
  if ! splice "$file" "$block_file" > "$out"; then
    return 5
  fi
  if cmp -s "$file" "$out"; then
    return 0
  fi
  if [ "$dry_run" -eq 1 ]; then
    echo "Would update $file"
    return 1
  fi
  mv "$out" "$file"
  echo "Updated $file"
  return 0
}

rc=0
process "$CLAUDE_SRC" "$claude_block_file" || rc=$?

if [ "$rc" -ne 0 ] && [ "$dry_run" -eq 1 ]; then
  exit 1
elif [ "$rc" -ne 0 ]; then
  exit "$rc"
fi

if [ "$dry_run" -eq 1 ]; then
  echo "No changes (mcp-allow block in sync)"
fi
exit 0
