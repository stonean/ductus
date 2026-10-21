#!/usr/bin/env bash
# Test surface for scripts/gen-configure-mcp.sh — the wildcard-block splice
# targets (Antigravity from spec 028, OpenCode from spec 032).
#
# Coverage:
#   A. `--dry-run` reports all four configure sources in sync (no drift)
#   B. antigravity.md's generated block is the single `mcp(ductus/*)` wildcard
#   C. claude.md / auggie.md still carry their per-tool blocks (regression)
#   D. opencode.md's generated block is the single `"ductus*": "allow"` glob
#   E. a configure file whose markers are swapped or repeated is refused for
#      that reason and left byte-identical (the splice would drop every line
#      after a misplaced start, or between two starts)
#
# Usage: scripts/tests/test-gen-configure-mcp.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
GEN="$REPO_ROOT/scripts/gen-configure-mcp.sh"
ANTIGRAVITY="$REPO_ROOT/framework/bootstrap/configure/antigravity.md"
CLAUDE="$REPO_ROOT/framework/bootstrap/configure/claude.md"
AUGGIE="$REPO_ROOT/framework/bootstrap/configure/auggie.md"
OPENCODE="$REPO_ROOT/framework/bootstrap/configure/opencode.md"

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

# Extract the lines between the generated:mcp-allow markers of a file.
mcp_block() {
  awk '/generated:mcp-allow:start/{p=1;next} /generated:mcp-allow:end/{p=0} p' "$1"
}

echo "Running gen-configure-mcp tests..."

# A. all four sources in sync
if "$GEN" --dry-run >/dev/null 2>&1; then
  pass "A: gen-configure-mcp --dry-run is in sync"
else
  fail "A: --dry-run reports drift — run scripts/gen-configure-mcp.sh and commit"
fi

# B. antigravity block is exactly one mcp(ductus/*) entry
ag_block="$(mcp_block "$ANTIGRAVITY")"
if printf '%s\n' "$ag_block" | grep -qF -- '- `mcp(ductus/*)`' \
   && [ "$(printf '%s\n' "$ag_block" | grep -c '`')" -eq 1 ]; then
  pass "B: antigravity.md block is the single mcp(ductus/*) wildcard"
else
  fail "B: antigravity.md block unexpected: $(printf '%s' "$ag_block" | tr '\n' '|')"
fi

# C. claude / auggie regression — per-tool blocks still present
if mcp_block "$CLAUDE" | grep -qF -- '- `mcp__ductus__read-spec`'; then
  pass "C: claude.md per-tool block intact"
else
  fail "C: claude.md per-tool block missing or changed"
fi
if mcp_block "$AUGGIE" | grep -qF -- 'mcp:ductus:read-spec'; then
  pass "C: auggie.md per-tool block intact"
else
  fail "C: auggie.md per-tool block missing or changed"
fi

# D. opencode block is exactly one `"ductus*": "allow"` entry
oc_block="$(mcp_block "$OPENCODE")"
if printf '%s\n' "$oc_block" | grep -qF -- '- `"ductus*": "allow"`'; then
  pass 'D: opencode.md block is the single "ductus*": "allow" glob'
else
  fail "D: opencode.md block unexpected: $(printf '%s' "$oc_block" | tr '\n' '|')"
fi

# E. malformed markers: run the generator on a scratch copy, where it
# resolves its paths from its own location, and check it refuses the file
# and writes nothing to it.
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
mkdir -p "$scratch/scripts" "$scratch/framework/bootstrap/configure"
cp "$GEN" "$scratch/scripts/"
cp "$REPO_ROOT/framework/runtime-tools.txt" "$scratch/framework/"
cp "$CLAUDE" "$AUGGIE" "$ANTIGRAVITY" "$OPENCODE" "$scratch/framework/bootstrap/configure/"
# Control: the scratch tree is complete, so a refusal below is the marker's.
if "$scratch/scripts/gen-configure-mcp.sh" --dry-run >/dev/null 2>&1; then
  pass "E: the scratch copy is in sync before any marker is changed"
else
  fail "E: the scratch copy fails before any marker is changed, so the cases below prove nothing"
fi
for case_name in swapped repeated; do
  cp "$CLAUDE" "$AUGGIE" "$ANTIGRAVITY" "$OPENCODE" "$scratch/framework/bootstrap/configure/"
  target="$scratch/framework/bootstrap/configure/claude.md"
  if [ "$case_name" = swapped ]; then
    sed -e 's/generated:mcp-allow:start/generated:mcp-allow:TMP/' \
        -e 's/generated:mcp-allow:end/generated:mcp-allow:start/' \
        -e 's/generated:mcp-allow:TMP/generated:mcp-allow:end/' "$CLAUDE" > "$target"
    reason='comes before the start marker'
  else
    awk '/^2\. /{print "   <!-- generated:mcp-allow:start -->"} {print}' "$CLAUDE" > "$target"
    reason='appears 2 times'
  fi
  cp "$target" "$scratch/before.md"
  if "$scratch/scripts/gen-configure-mcp.sh" >/dev/null 2>"$scratch/err"; then
    fail "E: a $case_name marker was not refused"
  elif ! grep -qF "$reason" "$scratch/err"; then
    fail "E: a $case_name marker was refused for another reason: $(head -n 1 "$scratch/err")"
  elif ! cmp -s "$target" "$scratch/before.md"; then
    fail "E: a $case_name marker was refused, but the file was rewritten"
  else
    pass "E: a $case_name marker is refused and the file left as it was"
  fi
done

if [ "$failures" -gt 0 ]; then
  echo "$failures test(s) failed" >&2
  exit 1
fi
echo "All gen-configure-mcp tests passed"
