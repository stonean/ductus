#!/usr/bin/env bash
# Test surface for install.sh's source resolution (spec 061).
#
# The installer is fetched and piped to `sh` by people who cannot see its
# logic, and every check it makes on the source guards against a silent
# outcome: a bootstrap from the wrong release, or a file written for a ref
# that should have halted. So each case runs the real installer in a fresh
# git repository with a stubbed `curl` first on PATH, and asserts both what it
# fetched and, on every halt, that it wrote nothing.
#
# Coverage:
#   A. the default resolves the tag the latest-release Location names
#   B. --ref=main fetches main, and never asks for the latest release
#   C. --ref=<tag> fetches that tag
#   D. --ref before the agent key is accepted
#   E. --ref after the agent key is accepted
#   F. --ref=latest resolves the latest release and is named in the next command
#   G. a tag exactly at the release floor is accepted
#   H. the floor compares numerically, not lexically (0.100.0 > 0.55.0)
#   I. a latest-release response with no Location halts, writing nothing
#   J. a latest release outside the ductus-v scheme halts, writing nothing
#   K. an unreachable latest release halts and does not fall back to main
#   L. a latest release below the floor halts with the pre-publication message
#   M. a named tag below the floor halts before fetching anything
#   N. a named tag that 404s halts as "does not exist", writing nothing
#   O. an empty --ref= halts before any network call
#   P. an unknown --ref value halts before any network call
#   Q. a repeated --ref halts naming every value
#   R. an unknown flag halts
#   S. DUCTUS_REPO names the fork every fetch goes to, is announced, and the
#      next step says to export it to the agent
#   T. an empty DUCTUS_REPO is the canonical origin, unannounced
#   U. the pi agent installs .pi/prompts/ductus.md and writes no settings file
#
# Each assertion was shown to fail against an installer with its check
# removed before it was trusted (spec 061, task 5).
#
# Usage: scripts/tests/test-install.sh
#   INSTALLER=/path/to/install.sh scripts/tests/test-install.sh   # a variant

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
INSTALLER="${INSTALLER:-$REPO_ROOT/install.sh}"

if [ ! -f "$INSTALLER" ]; then
  echo "test-install: installer not found at $INSTALLER — nothing to test, which is not a pass" >&2
  exit 1
fi

# The floor under test, read from the installer rather than restated here, so
# case G follows the value the installer actually carries.
FLOOR="$(sed -n 's/^REF_FLOOR="\([0-9.]*\)"$/\1/p' "$INSTALLER")"
if [ -z "$FLOOR" ]; then
  echo "test-install: no REF_FLOOR=\"X.Y.Z\" line in $INSTALLER — the floor cannot be tested" >&2
  exit 1
fi

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# The stub answers the two requests the installer makes:
#   - a HEAD (-I) of releases/latest: a 302 whose Location is $STUB_LOCATION,
#     a 200 with no Location when $STUB_LOCATION is empty, or a transport
#     failure when $STUB_LATEST_FAIL is set;
#   - a GET of a raw bootstrap URL with -o/-w: a 404 when the URL's ref is
#     $STUB_MISSING_REF, else a well-formed bootstrap and 200.
# Every URL requested is appended to $STUB_LOG.
mkdir -p "$WORK/bin"
cat > "$WORK/bin/curl" <<'STUB'
#!/bin/sh
head=0; out=""; fmt=""; url=""
while [ $# -gt 0 ]; do
  case "$1" in
    -o) out="$2"; shift ;;
    -w) fmt="$2"; shift ;;
    --proto) shift ;;
    --*) ;;
    -*I*) head=1 ;;
    -*) ;;
    *) url="$1" ;;
  esac
  shift
done
printf '%s\n' "$url" >> "$STUB_LOG"
if [ "$head" -eq 1 ]; then
  if [ -n "${STUB_LATEST_FAIL:-}" ]; then
    echo "curl: (6) Could not resolve host: github.com" >&2
    exit 6
  fi
  if [ -n "${STUB_LOCATION:-}" ]; then
    printf 'HTTP/2 302\r\nlocation: %s\r\n\r\n' "$STUB_LOCATION"
  else
    printf 'HTTP/2 200\r\ncontent-type: text/html\r\n\r\n'
  fi
  exit 0
fi
ref="${url#https://raw.githubusercontent.com/stonean/ductus/}"
ref="${ref%%/*}"
if [ -n "${STUB_MISSING_REF:-}" ] && [ "$ref" = "$STUB_MISSING_REF" ]; then
  printf '404: Not Found' > "$out"
  code=404
else
  printf -- '---\ndescription: stub bootstrap at %s\n---\n\n# ductus\n' "$ref" > "$out"
  code=200
fi
[ "$fmt" = '%{http_code}' ] && printf '%s' "$code"
exit 0
STUB
chmod +x "$WORK/bin/curl"

LATEST="https://github.com/stonean/ductus/releases/latest"
TAG_URL="https://github.com/stonean/ductus/releases/tag"

# run CASE [ARG...] — run the installer in a fresh repository. Leaves the exit
# status in $rc, the combined output in $WORK/CASE.out, and the requested URLs
# in $WORK/CASE.log. The STUB_* variables in the caller's environment steer
# the stub.
run() {
  name="$1"; shift
  dir="$WORK/$name"
  mkdir -p "$dir"
  git -C "$dir" init -q
  : > "$WORK/$name.log"
  set +e
  (cd "$dir" && PATH="$WORK/bin:$PATH" STUB_LOG="$WORK/$name.log" sh "$INSTALLER" "$@") \
    > "$WORK/$name.out" 2>&1
  rc=$?
  set -e
}

out() { cat "$WORK/$1.out"; }
fetched() { grep -qF -- "$2" "$WORK/$1.log"; }
requests() { grep -c . "$WORK/$1.log" || true; }
# Nothing but the repository itself: no bootstrap, no settings file.
wrote_nothing() { [ -z "$(cd "$WORK/$1" && find . -path ./.git -prune -o -type f -print)" ]; }

echo "Running install.sh tests (floor ductus-v$FLOOR)..."
unset STUB_LOCATION STUB_LATEST_FAIL STUB_MISSING_REF

# A. the default resolves the tag the Location names, and fetches that tag.
STUB_LOCATION="$TAG_URL/ductus-v99.0.0" run a-default
if [ "$rc" -eq 0 ] && fetched a-default "$LATEST" \
  && fetched a-default "/ductus-v99.0.0/framework/bootstrap/ductus.md" \
  && [ -f "$WORK/a-default/.claude/commands/ductus.md" ] \
  && out a-default | grep -qF "from latest release ductus-v99.0.0" \
  && out a-default | grep -qF "now run '/ductus <project-name>'"; then
  pass "A: the default resolves the latest release's tag and fetches it"
else
  fail "A: default resolution (rc=$rc): $(out a-default)"
fi

# B. --ref=main fetches main and never resolves the latest release.
STUB_LOCATION="$TAG_URL/ductus-v99.0.0" run b-main --ref=main
if [ "$rc" -eq 0 ] && fetched b-main "/main/framework/bootstrap/ductus.md" \
  && ! fetched b-main "$LATEST" \
  && out b-main | grep -qF "'/ductus --ref=main <project-name>'"; then
  pass "B: --ref=main fetches main and names /ductus --ref=main"
else
  fail "B: --ref=main (rc=$rc): $(out b-main)"
fi

# C. --ref=<tag> fetches that tag.
run c-tag --ref=ductus-v99.1.2
if [ "$rc" -eq 0 ] && fetched c-tag "/ductus-v99.1.2/framework/bootstrap/ductus.md" \
  && out c-tag | grep -qF "'/ductus --ref=ductus-v99.1.2 <project-name>'"; then
  pass "C: --ref=<tag> fetches the tag and names /ductus --ref=<tag>"
else
  fail "C: --ref=<tag> (rc=$rc): $(out c-tag)"
fi

# D. --ref before the agent key.
run d-before --ref=main auggie
if [ "$rc" -eq 0 ] && [ -f "$WORK/d-before/.augment/commands/ductus.md" ]; then
  pass "D: --ref before the agent key is accepted"
else
  fail "D: --ref before the agent (rc=$rc): $(out d-before)"
fi

# E. --ref after the agent key.
run e-after opencode --ref=main
if [ "$rc" -eq 0 ] && [ -f "$WORK/e-after/.opencode/command/ductus.md" ]; then
  pass "E: --ref after the agent key is accepted"
else
  fail "E: --ref after the agent (rc=$rc): $(out e-after)"
fi

# F. --ref=latest is the default spelled out, and is named in the next command.
STUB_LOCATION="$TAG_URL/ductus-v99.0.0" run f-latest --ref=latest
if [ "$rc" -eq 0 ] && fetched f-latest "/ductus-v99.0.0/framework/bootstrap/ductus.md" \
  && out f-latest | grep -qF "'/ductus --ref=latest <project-name>'"; then
  pass "F: --ref=latest resolves the latest release"
else
  fail "F: --ref=latest (rc=$rc): $(out f-latest)"
fi

# G. a tag exactly at the floor is not below it.
run g-at-floor "--ref=ductus-v$FLOOR"
if [ "$rc" -eq 0 ] && fetched g-at-floor "/ductus-v$FLOOR/framework/bootstrap/ductus.md"; then
  pass "G: a tag at the floor is accepted"
else
  fail "G: at the floor (rc=$rc): $(out g-at-floor)"
fi

# H. numeric, not lexical: 0.100.0 sorts below 0.55.0 as text.
run h-numeric --ref=ductus-v0.100.0
if [ "$rc" -eq 0 ] && fetched h-numeric "/ductus-v0.100.0/framework/bootstrap/ductus.md"; then
  pass "H: the floor comparison is numeric"
else
  fail "H: numeric comparison (rc=$rc): $(out h-numeric)"
fi

# I. no Location: halt, name what came back, write nothing, never fetch main.
STUB_LOCATION="" run i-no-location
if [ "$rc" -ne 0 ] && wrote_nothing i-no-location \
  && out i-no-location | grep -qF "no Location header" \
  && ! fetched i-no-location "/main/"; then
  pass "I: a missing Location halts and writes nothing"
else
  fail "I: missing Location (rc=$rc): $(out i-no-location)"
fi

# J. a tag outside the ductus-v scheme: halt, write nothing.
STUB_LOCATION="$TAG_URL/v9.0.0" run j-bad-scheme
if [ "$rc" -ne 0 ] && wrote_nothing j-bad-scheme \
  && out j-bad-scheme | grep -qF "$TAG_URL/v9.0.0" \
  && ! fetched j-bad-scheme "/main/"; then
  pass "J: a latest release outside ductus-v halts and writes nothing"
else
  fail "J: out-of-scheme tag (rc=$rc): $(out j-bad-scheme)"
fi

# K. the latest release unreachable: halt, and do not fall back to main.
STUB_LATEST_FAIL=1 run k-unreachable
if [ "$rc" -ne 0 ] && wrote_nothing k-unreachable \
  && [ "$(requests k-unreachable)" -eq 1 ] \
  && out k-unreachable | grep -qF "does not fall back to main"; then
  pass "K: an unreachable latest release halts without falling back to main"
else
  fail "K: unreachable latest (rc=$rc): $(out k-unreachable)"
fi

# L. the latest release below the floor: the pre-publication message.
STUB_LOCATION="$TAG_URL/ductus-v0.54.2" run l-latest-below
if [ "$rc" -ne 0 ] && wrote_nothing l-latest-below \
  && out l-latest-below | grep -qF "may still be publishing" \
  && out l-latest-below | grep -qF "ductus-v$FLOOR"; then
  pass "L: a latest release below the floor halts with the pre-publication message"
else
  fail "L: latest below the floor (rc=$rc): $(out l-latest-below)"
fi

# M. a named tag below the floor: halt naming both, before fetching anything.
run m-named-below --ref=ductus-v0.54.2
if [ "$rc" -ne 0 ] && wrote_nothing m-named-below \
  && [ "$(requests m-named-below)" -eq 0 ] \
  && out m-named-below | grep -qF "ductus-v0.54.2 is older than ductus-v$FLOOR"; then
  pass "M: a named tag below the floor halts before any fetch"
else
  fail "M: named tag below the floor (rc=$rc): $(out m-named-below)"
fi

# N. a named tag that does not exist: its bootstrap 404s.
STUB_MISSING_REF=ductus-v99.9.9 run n-missing --ref=ductus-v99.9.9
if [ "$rc" -ne 0 ] && wrote_nothing n-missing \
  && out n-missing | grep -qF "tag ductus-v99.9.9 does not exist"; then
  pass "N: a nonexistent tag halts as 'does not exist' and writes nothing"
else
  fail "N: nonexistent tag (rc=$rc): $(out n-missing)"
fi

# O. an empty value: halt before any network call.
run o-empty --ref=
if [ "$rc" -ne 0 ] && wrote_nothing o-empty && [ "$(requests o-empty)" -eq 0 ] \
  && out o-empty | grep -qF 'invalid --ref ""'; then
  pass "O: --ref= halts before any network call"
else
  fail "O: empty value (rc=$rc): $(out o-empty)"
fi

# P. an unknown value: halt before any network call.
run p-unknown --ref=v1.2.3
if [ "$rc" -ne 0 ] && wrote_nothing p-unknown && [ "$(requests p-unknown)" -eq 0 ] \
  && out p-unknown | grep -qF 'invalid --ref "v1.2.3"'; then
  pass "P: an unknown value halts before any network call"
else
  fail "P: unknown value (rc=$rc): $(out p-unknown)"
fi

# Q. a repeated --ref: halt naming every value.
run q-repeated --ref=main claude --ref=ductus-v99.0.0
if [ "$rc" -ne 0 ] && wrote_nothing q-repeated && [ "$(requests q-repeated)" -eq 0 ] \
  && out q-repeated | grep -qF '"main", "ductus-v99.0.0"'; then
  pass "Q: a repeated --ref halts naming every value"
else
  fail "Q: repeated --ref (rc=$rc): $(out q-repeated)"
fi

# R. an unknown flag halts.
run r-flag --frob
if [ "$rc" -ne 0 ] && wrote_nothing r-flag && out r-flag | grep -qF "unknown flag '--frob'"; then
  pass "R: an unknown flag halts"
else
  fail "R: unknown flag (rc=$rc): $(out r-flag)"
fi

# S. DUCTUS_REPO names the fork both fetches go to, and the installer says so:
# a variable left set in a shell profile must not silently change the origin.
STUB_LOCATION="https://github.com/fork/ductus/releases/tag/ductus-v99.0.0" \
  DUCTUS_REPO=fork/ductus run s-fork
if [ "$rc" -eq 0 ] && fetched s-fork "https://github.com/fork/ductus/releases/latest" \
  && fetched s-fork "https://raw.githubusercontent.com/fork/ductus/ductus-v99.0.0/framework/bootstrap/ductus.md" \
  && ! fetched s-fork "stonean/ductus" \
  && out s-fork | grep -qF "source repository is fork/ductus (from DUCTUS_REPO)" \
  && out s-fork | grep -qF "start your agent with DUCTUS_REPO=fork/ductus exported"; then
  pass "S: DUCTUS_REPO sends every fetch to the fork, announces it, and says to export it"
else
  fail "S: DUCTUS_REPO fork (rc=$rc): $(out s-fork)"
fi

# T. an empty DUCTUS_REPO is the canonical origin, and nothing is announced.
STUB_LOCATION="$TAG_URL/ductus-v99.0.0" DUCTUS_REPO="" run t-empty
if [ "$rc" -eq 0 ] && fetched t-empty "$LATEST" \
  && ! out t-empty | grep -qF "source repository is" \
  && ! out t-empty | grep -qF "DUCTUS_REPO="; then
  pass "T: an empty DUCTUS_REPO is the canonical origin, unannounced"
else
  fail "T: empty DUCTUS_REPO (rc=$rc): $(out t-empty)"
fi

# U. pi installs the bootstrap as a prompt template and seeds no settings:
# Pi has no permission-gating settings to pre-authorize.
run u-pi --ref=main pi
if [ "$rc" -eq 0 ] && [ -f "$WORK/u-pi/.pi/prompts/ductus.md" ] \
  && [ "$(cd "$WORK/u-pi" && find . -path ./.git -prune -o -type f -print)" = "./.pi/prompts/ductus.md" ]; then
  pass "U: pi installs .pi/prompts/ductus.md and nothing else"
else
  fail "U: pi agent (rc=$rc): $(out u-pi)"
fi

echo
if [ "$failures" -gt 0 ]; then
  echo "$failures test(s) failed" >&2
  exit 1
fi
echo "All install.sh tests passed."
