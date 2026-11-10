#!/usr/bin/env bash
# Test surface for scripts/gen-claude-commands.sh's runtime check (spec 080,
# task 26): a runtime whose `apply-manifest --help` does not name
# tool-search-line halts the generator before it writes anything, and one that
# names it is let through.
#
# The generator runs from a scratch copy of the tree, so nothing here writes
# to this checkout. Each stub runtime logs its arguments, so the test can see
# whether the generator went past the check to build the Claude copies.
#
# Coverage:
#   A. a stub whose help lacks the field: write mode exits non-zero, names the
#      runtime and the field, writes no generated directory, and never calls
#      apply-manifest to build
#   B. the same stub under --check refuses the same way
#   C. a stub whose help names the field passes the check and is called to
#      build the copies (its build writes nothing, so --check reports drift)
#   D. this checkout's release build passes the check, so a rewording of its
#      description that drops the field fails here rather than at a commit
#   E. a runtime that never answers is stopped once DUCTUS_GEN_TIMEOUT_S
#      passes, and the generator exits non-zero naming it (BE-TIMEOUT-001)
#
# Usage: scripts/tests/test-gen-claude-commands.sh
# D needs a release build: runtime/target/release/ductus, or DUCTUS_BIN.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
REAL_BIN="${DUCTUS_BIN:-$REPO_ROOT/runtime/target/release/ductus}"
REFUSAL="predates apply-manifest's tool-search-line"

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

work="$(mktemp -d -t test-gen-claude-commands-XXXXXX)"
trap 'rm -rf "$work"' EXIT

# A scratch root holding the generator and the sources it reads.
scratch_root() {
  local root="$work/$1"
  mkdir -p "$root/scripts"
  cp "$REPO_ROOT/scripts/gen-claude-commands.sh" "$root/scripts/"
  cp -R "$REPO_ROOT/framework" "$root/framework"
  printf '%s\n' "$root"
}

# A stub runtime at $work/$1 whose `apply-manifest --help` prints $2, and
# which logs every call's arguments to $work/$1.log and otherwise does nothing.
stub_runtime() {
  local bin="$work/$1"
  cat > "$bin" <<EOF
#!/usr/bin/env bash
printf '%s\n' "\$*" >> "$bin.log"
if [ "\$1" = apply-manifest ] && [ "\${2:-}" = --help ]; then
  printf '%s\n' "$2"
fi
exit 0
EOF
  chmod +x "$bin"
  : > "$bin.log"
  printf '%s\n' "$bin"
}

echo "Running gen-claude-commands tests..."

stale="$(stub_runtime stale 'Strategy-aware bulk substitute + write driven by a manifest.')"
current="$(stub_runtime current 'Strategy-aware bulk substitute + write driven by a manifest. An entry setting tool-search-line ends its preamble with a ToolSearch line.')"

# A. write mode with the stale stub
root="$(scratch_root a)"
if out="$(DUCTUS_BIN="$stale" "$root/scripts/gen-claude-commands.sh" 2>&1)"; then
  fail "A: write mode with a stale runtime exited 0"
else
  case "$out" in
    *"$stale"*"$REFUSAL"*) pass "A: write mode refuses a stale runtime, naming it" ;;
    *) fail "A: write mode failed without the refusal: $out" ;;
  esac
fi
written=""
for dir in .claude .pi .ductus; do
  if [ -e "$root/$dir" ]; then written="$written $dir"; fi
done
if [ -z "$written" ]; then
  pass "A: nothing was written"
else
  fail "A: a refused run wrote:$written"
fi
if grep -q -- '--entries-json' "$stale.log"; then
  fail "A: the stale runtime was called to build the copies"
else
  pass "A: the stale runtime was never called to build"
fi

# B. --check with the stale stub
: > "$stale.log"
root="$(scratch_root b)"
if out="$(DUCTUS_BIN="$stale" "$root/scripts/gen-claude-commands.sh" --check 2>&1)"; then
  fail "B: --check with a stale runtime exited 0"
else
  case "$out" in
    *"$REFUSAL"*) pass "B: --check refuses a stale runtime" ;;
    *) fail "B: --check failed without the refusal: $out" ;;
  esac
fi
if grep -q -- '--entries-json' "$stale.log"; then
  fail "B: the stale runtime was called to build the copies"
else
  pass "B: the stale runtime was never called to build"
fi

# C. --check with the current stub
root="$(scratch_root c)"
rc=0
out="$(DUCTUS_BIN="$current" "$root/scripts/gen-claude-commands.sh" --check 2>&1)" || rc=$?
case "$rc:$out" in
  *"$REFUSAL"*) fail "C: a runtime that names the field was refused" ;;
  "1:"*"missing in DEST"*) pass "C: a runtime that names the field passes the check" ;;
  *) fail "C: --check gave no drift report to read (exit $rc): $out" ;;
esac
if grep -q -- '--entries-json' "$current.log"; then
  pass "C: the runtime was called to build the copies"
else
  fail "C: the runtime was never called to build the copies"
fi

# D. this checkout's release build
if [ ! -x "$REAL_BIN" ]; then
  fail "D: no release build at $REAL_BIN; build it with (cd runtime && cargo build --release)"
else
  root="$(scratch_root d)"
  rc=0
  out="$(DUCTUS_BIN="$REAL_BIN" "$root/scripts/gen-claude-commands.sh" --check 2>&1)" || rc=$?
  case "$rc:$out" in
    *"$REFUSAL"*) fail "D: the release build at $REAL_BIN was refused" ;;
    "1:"*"missing in DEST"*) pass "D: the release build passes the check and builds the copies" ;;
    *) fail "D: --check with the release build gave no drift report to read (exit $rc): $out" ;;
  esac
fi

# E. a runtime that never answers
hung="$work/hung"
cat > "$hung" <<'EOF2'
#!/usr/bin/env bash
# One process, as the runtime is, so stopping it closes its output.
exec sleep 60
EOF2
chmod +x "$hung"
root="$(scratch_root e)"
started=$SECONDS
if out="$(DUCTUS_GEN_TIMEOUT_S=1 DUCTUS_BIN="$hung" "$root/scripts/gen-claude-commands.sh" 2>&1)"; then
  fail "E: a runtime that never answers exited 0"
else
  elapsed=$((SECONDS - started))
  case "$out" in
    *"$hung did not answer within 1s"*)
      if [ "$elapsed" -lt 30 ]; then
        pass "E: a runtime that never answers is stopped at the bound, named"
      else
        fail "E: the generator waited ${elapsed}s, past the 1s bound"
      fi
      ;;
    *) fail "E: the generator failed without naming the bound: $out" ;;
  esac
fi

if [ "$failures" -ne 0 ]; then
  echo "$failures failure(s)" >&2
  exit 1
fi
echo "All gen-claude-commands tests passed."
