#!/usr/bin/env bash
# Test surface for scripts/audit/host-namespace-parity.sh (Family 17).
#
# The family resolves the namespace the runtime renders the way `Host::load`
# does and compares it with the installed command namespaces. Since `Host::load`
# halts on a config that does not parse, so does the family: falling back to the
# repository's basename would compare a namespace nothing renders, and where
# the basename happens to match it would pass (022 task 29).
#
# Coverage:
#   A. a config naming the installed namespace passes (control)
#   B. no config at all falls back to the basename and passes when it matches
#   C. a `.ductus/config.toml` that does not parse fails, naming the file, even
#      where the basename matches the installed namespace
#   D. a malformed legacy `.govern/config.toml`, the tier in use, fails naming it
#
# Usage: scripts/tests/test-host-namespace-parity.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# make_tree CASE BASENAME — a fixture repository whose directory is named
# BASENAME, under a directory of its own for CASE, holding the family, its lib,
# the runtime sources its contracts are asserted against, the Agent Registry it
# derives the agent directories from, one command source, and an installed
# `.claude/commands/acme/` namespace. Prints the directory.
make_tree() {
  local dir="$WORK/$1/$2"
  mkdir -p "$dir/scripts/audit" "$dir/runtime/src/schema" "$dir/framework/bootstrap" \
    "$dir/framework/commands" "$dir/.claude/commands/acme"
  cp "$REPO_ROOT/scripts/audit/host-namespace-parity.sh" "$REPO_ROOT/scripts/audit/lib.sh" "$dir/scripts/audit/"
  cp "$REPO_ROOT/runtime/src/host.rs" "$dir/runtime/src/"
  cp "$REPO_ROOT/runtime/src/schema/paths.rs" "$dir/runtime/src/schema/"
  cp "$REPO_ROOT/framework/bootstrap/ductus.md" "$dir/framework/bootstrap/"
  printf '# target\n' > "$dir/framework/commands/target.md"
  printf '# target\n' > "$dir/.claude/commands/acme/target.md"
  echo "$dir"
}

# run_family DIR — run the fixture's copy; sets out and code.
run_family() {
  set +e
  out="$("$BASH" "$1/scripts/audit/host-namespace-parity.sh" 2> /dev/null)"
  code=$?
  set -e
}

# expect NAME CODE PATTERN — the last run exited CODE and its findings match
# PATTERN (an extended regex; empty means no finding at all).
expect() {
  if [ "$code" -ne "$2" ]; then fail "$1: exit $code, want $2: $out"; return; fi
  if [ -z "$3" ]; then
    if [ -n "$out" ]; then fail "$1: want no finding: $out"; else pass "$1"; fi
    return
  fi
  if printf '%s' "$out" | grep -qE -- "$3"; then pass "$1"; else fail "$1: no finding matched /$3/: $out"; fi
}

echo "Running host-namespace-parity tests..."

dir="$(make_tree a other)"
mkdir -p "$dir/.ductus"
printf '[host]\nproject = "acme"\n' > "$dir/.ductus/config.toml"
run_family "$dir"
expect "A: a config naming the installed namespace passes" 0 ""

dir="$(make_tree b acme)"
run_family "$dir"
expect "B: no config falls back to a matching basename and passes" 0 ""

dir="$(make_tree c acme)"
mkdir -p "$dir/.ductus"
printf '[host\nproject = "acme"\n' > "$dir/.ductus/config.toml"
run_family "$dir"
expect "C: an unparseable .ductus/config.toml fails naming it, the basename matching" 1 \
  '^host-namespace-parity \| \.ductus/config\.toml \| cannot be read or does not parse as TOML'

dir="$(make_tree d acme)"
mkdir -p "$dir/.govern"
printf 'project = = "acme"\n' > "$dir/.govern/config.toml"
run_family "$dir"
expect "D: an unparseable legacy .govern/config.toml fails naming it" 1 \
  '^host-namespace-parity \| \.govern/config\.toml \| cannot be read or does not parse as TOML'

if [ "$failures" -ne 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
echo "All host-namespace-parity cases passed."
