#!/usr/bin/env bash
# Report whether the runtime is current, and fail when it is not.
#
# Nothing else keeps it current. Before this check, Cargo.lock had drifted 121
# packages behind, two direct dependencies sat a major version back, the
# pinned toolchain was two releases old, and four rustls advisories were found
# only because someone ran `cargo audit` by hand — the push workflow runs it
# only when a commit touches runtime/. Each of those is a chore that depends on
# someone remembering it, so this asks all four questions on a schedule
# (.github/workflows/runtime-currency.yml) and from a contributor's shell:
#
#   1. lockfile     — `cargo update --dry-run` would change Cargo.lock.
#   2. majors       — a direct dependency has a newer, semver-incompatible
#                     release (moving it is a Cargo.toml and code change).
#   3. toolchain    — runtime/rust-toolchain.toml pins an older channel than
#                     the latest stable Rust.
#   4. advisories   — `cargo audit` reports a vulnerability (runtime/audit.toml
#                     still governs what is ignored).
#
# A compatible update held back only by the declared `rust-version` (MSRV) is
# reported, never failed: the MSRV is a deliberate floor, raised on purpose.
#
# Exit status: 1 when any check found the runtime behind; otherwise 2 when any
# check could not run (no network, no cargo-audit), so an unexamined subject
# never reads as current; 0 only when every check ran and found nothing.
#
# Usage: scripts/check-runtime-currency.sh   (from anywhere in the repo)

set -uo pipefail

root=$(git rev-parse --show-toplevel)
runtime="$root/runtime"
behind=0
unexamined=0

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

report() { printf '%s\n' "$*"; }

# --- 1 + 2: lockfile and majors, from one verbose dry run --------------------
if (cd "$runtime" && cargo update --dry-run --verbose) >"$tmp/update.out" 2>&1; then
  moves=$(grep -E '^[[:space:]]+(Updating|Adding|Removing|Downgrading) [A-Za-z0-9_-]+ v' "$tmp/update.out" |
    sed -E 's/^[[:space:]]+//')
  if [ -n "$moves" ]; then
    behind=1
    report "lockfile: behind — \`cargo update\` would make $(printf '%s\n' "$moves" | wc -l | tr -d ' ') change(s):"
    printf '%s\n' "$moves" | sed 's/^/  /'
  else
    report "lockfile: current"
  fi

  if (cd "$runtime" && cargo metadata --no-deps --format-version 1) >"$tmp/metadata.json" 2>"$tmp/metadata.err" &&
    jq -r '.packages[0].dependencies[].name' "$tmp/metadata.json" | sort -u >"$tmp/direct.txt"; then
    majors=""
    msrv_held=""
    while IFS= read -r line; do
      name=$(printf '%s\n' "$line" | sed -E 's/^[[:space:]]*Unchanged ([^ ]+) .*/\1/')
      if printf '%s\n' "$line" | grep -q 'requires Rust'; then
        msrv_held+="  $(printf '%s\n' "$line" | sed -E 's/^[[:space:]]+//')"$'\n'
      elif grep -qx -- "$name" "$tmp/direct.txt"; then
        majors+="  $(printf '%s\n' "$line" | sed -E 's/^[[:space:]]+//')"$'\n'
      fi
    done < <(grep -E '^[[:space:]]+Unchanged [^ ]+ v[^ ]+ \(available: ' "$tmp/update.out")
    if [ -n "$majors" ]; then
      behind=1
      report "majors: behind — direct dependencies with a newer major release:"
      printf '%s' "$majors"
    else
      report "majors: current ($(wc -l <"$tmp/direct.txt" | tr -d ' ') direct dependencies examined)"
    fi
    if [ -n "$msrv_held" ]; then
      report "msrv: held back by the declared rust-version (reported, not failed):"
      printf '%s' "$msrv_held"
    fi
  else
    unexamined=1
    report "majors: NOT EXAMINED — cargo metadata or jq failed:"
    sed 's/^/  /' "$tmp/metadata.err"
  fi
else
  unexamined=1
  report "lockfile: NOT EXAMINED — cargo update --dry-run failed:"
  sed 's/^/  /' "$tmp/update.out"
  report "majors: NOT EXAMINED — it reads the same dry run"
fi

# --- 3: toolchain -------------------------------------------------------------
pinned=$(sed -nE 's/^channel = "([^"]+)"$/\1/p' "$runtime/rust-toolchain.toml")
if curl -fsSL https://static.rust-lang.org/dist/channel-rust-stable.toml -o "$tmp/channel.toml"; then
  stable=$(awk '/^\[pkg\.rust\]$/{f=1} f && /^version = /{print; exit}' "$tmp/channel.toml" |
    sed -E 's/^version = "([0-9]+\.[0-9]+\.[0-9]+).*/\1/')
  if [ -z "$pinned" ] || [ -z "$stable" ]; then
    unexamined=1
    report "toolchain: NOT EXAMINED — pinned '${pinned:-?}', latest stable '${stable:-?}'"
  elif [ "$(printf '%s\n%s\n' "$pinned" "$stable" | sort -V | tail -1)" != "$pinned" ]; then
    behind=1
    report "toolchain: behind — runtime/rust-toolchain.toml pins $pinned, latest stable is $stable"
  else
    report "toolchain: current ($pinned)"
  fi
else
  unexamined=1
  report "toolchain: NOT EXAMINED — could not fetch the stable channel manifest"
fi

# --- 4: advisories ------------------------------------------------------------
if (cd "$runtime" && cargo audit --version) >/dev/null 2>&1; then
  (cd "$runtime" && cargo audit) >"$tmp/audit.out" 2>&1
  status=$?
  if [ "$status" -eq 0 ]; then
    report "advisories: none ($(grep -oE 'for vulnerabilities \([0-9]+ crate dependencies\)' "$tmp/audit.out" | grep -oE '[0-9]+') crates scanned)"
  elif grep -q 'vulnerabilit' "$tmp/audit.out" && grep -qE '^ID:' "$tmp/audit.out"; then
    behind=1
    report "advisories: $(grep -cE '^ID:' "$tmp/audit.out") found by cargo audit:"
    grep -E '^(Crate|Version|ID|Solution):' "$tmp/audit.out" | sed 's/^/  /'
  else
    unexamined=1
    report "advisories: NOT EXAMINED — cargo audit exited $status:"
    tail -5 "$tmp/audit.out" | sed 's/^/  /'
  fi
else
  unexamined=1
  report "advisories: NOT EXAMINED — cargo-audit is not installed (cargo install cargo-audit --locked)"
fi

if [ "$behind" -eq 1 ]; then
  exit 1
elif [ "$unexamined" -eq 1 ]; then
  exit 2
fi
exit 0
