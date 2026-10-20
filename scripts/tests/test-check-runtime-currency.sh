#!/usr/bin/env bash
# Test surface for scripts/check-runtime-currency.sh's pins check (check 5).
#
# The check exists so that a pin falling behind its upstream turns the weekly
# run red. A comparison that silently stops comparing reads exactly like a pin
# that is current, so the only evidence it works is that it FAILS on each case
# it claims to catch, and reports NOT EXAMINED, never current, when it cannot
# look. Each case runs the check in a throwaway git repository against stubbed
# `gh` and `curl`, so nothing here touches the network.
#
# Coverage:
#   A. a fixture whose pins all match upstream is current (control); it also
#      holds a local reusable workflow and a commented `cargo install`, neither
#      of which is a pin
#   B. an action whose latest release is newer is behind
#   C. an action whose tag now names another commit is behind
#   D. an action referenced by tag rather than commit SHA is behind
#   E. an action pin with no release tag comment is behind
#   F. a taiki-e tool behind crates.io is behind
#   G. a taiki-e tool with no version is behind
#   H. a `cargo install` behind crates.io is behind
#   I. a `cargo install` with no --version is behind
#   J. a hash-pinned pip requirement behind PyPI is behind
#   K. a pip install without --require-hashes is behind
#   L. a toolchain input that differs from rust-toolchain.toml is behind
#   M. a GitHub lookup that fails is NOT EXAMINED (exit 2), never current
#   N. a registry lookup that fails is NOT EXAMINED (exit 2), never current
#   O. a tree with no workflow is NOT EXAMINED rather than vacuously current
#   P. a behind pin outranks an unexamined one (exit 1)
#   Q. the CI template's ductus release behind the latest release is behind
#   R. a DUCTUS_SHA256 that is not the release's published sha256 is behind
#   S. a DUCTUS_VERSION with no DUCTUS_SHA256 beside it is behind
#   T. a published sha256 that cannot be read is NOT EXAMINED (exit 2)
#   U. an unknown check name is refused
#
# Usage: scripts/tests/test-check-runtime-currency.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
CHECK="$REPO_ROOT/scripts/check-runtime-currency.sh"

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

sha() { printf '%040d' "$1"; }

# --- the stubs ----------------------------------------------------------------
mkdir -p "$WORK/bin"
cat >"$WORK/bin/gh" <<'STUB'
#!/usr/bin/env bash
# `gh api <path> [--jq <expr>]`, answered from $STUB_DIR/gh.tsv (path, value).
[ "${1:-}" = api ] || exit 1
awk -F'\t' -v p="${2:-}" '$1 == p { print $2; found = 1 } END { exit !found }' "$STUB_DIR/gh.tsv"
STUB
cat >"$WORK/bin/curl" <<'STUB'
#!/usr/bin/env bash
# Serves each https:// URL from $STUB_DIR/curl.tsv (url, file); unknown is a 404.
url=""
for a in "$@"; do case "$a" in https://*) url=$a ;; esac; done
file=$(awk -F'\t' -v u="$url" '$1 == u { print $2; found = 1 } END { exit !found }' "$STUB_DIR/curl.tsv") || exit 22
cat "$STUB_DIR/$file"
STUB
chmod +x "$WORK/bin/gh" "$WORK/bin/curl"

# --- the base fixture ---------------------------------------------------------
BASE="$WORK/base"
mkdir -p "$BASE/repo/runtime" "$BASE/repo/.github/workflows" "$BASE/repo/framework/templates/ci" "$BASE/stub"
printf '[toolchain]\nchannel = "1.99.1"\n' >"$BASE/repo/runtime/rust-toolchain.toml"
cat >"$BASE/repo/.github/workflows/ci.yml" <<EOF
name: fixture
on: push
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@$(sha 1) # v7.0.1
      - uses: dtolnay/rust-toolchain@$(sha 2) # v1
        with:
          toolchain: 1.99.1
      - uses: taiki-e/install-action@$(sha 3) # v2.87.29
        with:
          tool: cargo-audit@0.22.2
      - uses: actions/setup-node@$(sha 4) # v6.5.0
      # a cargo install in a comment is not a pin
      - run: |
          printf '%s\n' 'ziglang==0.17.0 --hash=sha256:$(printf '%064d' 9)' > r.txt
          pip3 install --require-hashes -r r.txt
          cargo install --locked cargo-zigbuild --version 0.23.4
  reuse:
    uses: ./.github/workflows/other.yml
EOF
cat >"$BASE/repo/framework/templates/ci/adopter.yml" <<EOF
name: template
on: push
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@$(sha 1) # v7.0.1
      - name: Install the ductus runtime
        env:
          DUCTUS_VERSION: 0.59.11
          DUCTUS_SHA256: $(printf '%064d' 7)
EOF
(cd "$BASE/repo" && git init -q && git add -A)

printf '%s\t%s\n' \
  repos/actions/checkout/commits/v7.0.1 "$(sha 1)" \
  repos/actions/checkout/releases/latest v7.0.1 \
  repos/dtolnay/rust-toolchain/commits/v1 "$(sha 2)" \
  repos/dtolnay/rust-toolchain/releases/latest v1 \
  repos/taiki-e/install-action/commits/v2.87.29 "$(sha 3)" \
  repos/taiki-e/install-action/releases/latest v2.87.29 \
  repos/actions/setup-node/commits/v6.5.0 "$(sha 4)" \
  repos/actions/setup-node/releases/latest v6.5.0 \
  repos/stonean/ductus/releases/latest ductus-v0.59.11 >"$BASE/stub/gh.tsv"
printf '%s\t%s\n' \
  https://crates.io/api/v1/crates/cargo-audit cargo-audit.json \
  https://crates.io/api/v1/crates/cargo-zigbuild cargo-zigbuild.json \
  https://pypi.org/pypi/ziglang/json ziglang.json \
  https://github.com/stonean/ductus/releases/download/ductus-v0.59.11/ductus-x86_64-unknown-linux-gnu.tar.gz.sha256 \
  ductus.sha256 >"$BASE/stub/curl.tsv"
printf '{"crate":{"max_stable_version":"0.22.2"}}\n' >"$BASE/stub/cargo-audit.json"
printf '{"crate":{"max_stable_version":"0.23.4"}}\n' >"$BASE/stub/cargo-zigbuild.json"
printf '{"info":{"version":"0.17.0"}}\n' >"$BASE/stub/ziglang.json"
printf '%064d  ductus-x86_64-unknown-linux-gnu.tar.gz\n' 7 >"$BASE/stub/ductus.sha256"

# probe <case> [sed-edit-of-a-fixture-file ...] runs the pins check in a copy of
# the base fixture. Each edit is `file|sed-expression`, its file relative to
# the case directory (repo/... or stub/...). Sets $out and $code.
probe() {
  case_dir="$WORK/$1"
  shift
  cp -R "$BASE" "$case_dir"
  for edit in "$@"; do
    f="$case_dir/${edit%%|*}"
    sed "${edit#*|}" "$f" >"$f.new" && mv "$f.new" "$f"
  done
  (cd "$case_dir/repo" && git add -A)
  code=0
  out=$(cd "$case_dir/repo" && STUB_DIR="$case_dir/stub" PATH="$WORK/bin:$PATH" bash "$CHECK" pins 2>&1) || code=$?
}

# expect <label> <exit-code> <substring> checks the last probe's result.
expect() {
  if [ "$code" -eq "$2" ] && printf '%s\n' "$out" | grep -qF -- "$3"; then
    pass "$1"
  else
    fail "$1 — exit $code, wanted $2 and '$3' in: $out"
  fi
}

echo "Running check-runtime-currency pins tests..."

probe a-control
expect "A: a fixture matching upstream is current" 0 \
  "pins: current (4 action pin(s) and 5 tool pin(s) across 2 file(s))"

probe b-release 'stub/gh.tsv|/setup-node\/releases\/latest/s|v6.5.0|v7.1.0|'
expect "B: a newer release is behind" 1 "actions/setup-node pins v6.5.0, latest release is v7.1.0"

probe c-moved "stub/gh.tsv|/dtolnay\/rust-toolchain\/commits\/v1/s|$(sha 2)|$(sha 5)|"
expect "C: a tag naming another commit is behind" 1 "but v1 now names $(sha 5)"

probe d-tag "repo/framework/templates/ci/adopter.yml|s|actions/checkout@[0-9]* # v7.0.1|actions/checkout@v7|"
expect "D: a tag reference is not a pin" 1 "actions/checkout@v7 is not pinned to a commit SHA"

probe e-nocomment 'repo/.github/workflows/ci.yml|s| # v6.5.0||'
expect "E: a pin with no tag comment is behind" 1 "has no release tag in a comment to compare"

probe f-tool 'stub/cargo-audit.json|s|0.22.2|0.23.0|'
expect "F: a taiki-e tool behind crates.io is behind" 1 "cargo-audit pins 0.22.2, latest is 0.23.0"

probe g-toolver 'repo/.github/workflows/ci.yml|s|cargo-audit@0.22.2|cargo-audit|'
expect "G: a taiki-e tool with no version is behind" 1 "tool cargo-audit has no pinned version"

probe h-crate 'stub/cargo-zigbuild.json|s|0.23.4|0.24.0|'
expect "H: a cargo install behind crates.io is behind" 1 "cargo-zigbuild pins 0.23.4, latest is 0.24.0"

probe i-nover 'repo/.github/workflows/ci.yml|s| --version 0.23.4||'
expect "I: a cargo install with no --version is behind" 1 "cargo install cargo-zigbuild has no pinned version"

probe j-pypi 'stub/ziglang.json|s|0.17.0|0.18.0|'
expect "J: a pip requirement behind PyPI is behind" 1 "ziglang pins 0.17.0, latest is 0.18.0"

probe k-nohash 'repo/.github/workflows/ci.yml|s|pip3 install --require-hashes -r r.txt|pip3 install ziglang|'
expect "K: a pip install without --require-hashes is behind" 1 "pip install without --require-hashes"

probe l-toolchain 'repo/.github/workflows/ci.yml|s|toolchain: 1.99.1|toolchain: 1.98.0|'
expect "L: a toolchain input off rust-toolchain.toml is behind" 1 \
  "toolchain: 1.98.0 differs from runtime/rust-toolchain.toml's 1.99.1"

probe m-github 'stub/gh.tsv|/install-action\/releases\/latest/d'
expect "M: a failed GitHub lookup is not examined" 2 "pins: NOT EXAMINED — 1 of"
if printf '%s\n' "$out" | grep -q 'pins: current'; then fail "M: a failed lookup still printed current"; fi

probe n-registry 'stub/curl.tsv|/cargo-audit/d'
expect "N: a failed registry lookup is not examined" 2 "cargo-audit 0.22.2"
if printf '%s\n' "$out" | grep -q 'pins: current'; then fail "N: a failed lookup still printed current"; fi

case_dir="$WORK/o-empty"
mkdir -p "$case_dir/runtime"
printf '[toolchain]\nchannel = "1.99.1"\n' >"$case_dir/runtime/rust-toolchain.toml"
(cd "$case_dir" && git init -q && git add -A)
code=0
out=$(cd "$case_dir" && STUB_DIR="$BASE/stub" PATH="$WORK/bin:$PATH" bash "$CHECK" pins 2>&1) || code=$?
expect "O: no workflow is not examined, not current" 2 "the enumeration is broken, not the pins"

probe p-both 'stub/cargo-audit.json|s|0.22.2|0.23.0|' 'stub/curl.tsv|/ziglang/d'
expect "P: behind outranks unexamined" 1 "pins: NOT EXAMINED — 1 of"

probe q-release 'stub/gh.tsv|/stonean\/ductus\/releases\/latest/s|ductus-v0.59.11|ductus-v0.60.0|'
expect "Q: a template release behind the latest release is behind" 1 "ductus pins 0.59.11, latest is 0.60.0"

probe r-hash 'stub/ductus.sha256|s|^0|1|'
expect "R: a hash the release did not publish is behind" 1 "is not the sha256 ductus 0.59.11 published"

probe s-nohash 'repo/framework/templates/ci/adopter.yml|/DUCTUS_SHA256/d'
expect "S: a release version with no hash is behind" 1 "ductus 0.59.11 has no DUCTUS_SHA256 beside it"

probe t-sidecar 'stub/curl.tsv|/ductus-v0.59.11/d'
expect "T: an unreadable published hash is not examined" 2 "could not read the release's published sha256"

code=0
out=$(cd "$BASE/repo" && bash "$CHECK" pinz 2>&1) || code=$?
expect "U: an unknown check is refused" 2 "unknown check: pinz"

echo
if [ "$failures" -ne 0 ]; then
  echo "$failures check-runtime-currency test(s) failed."
  exit 1
fi
echo "All check-runtime-currency tests passed."
