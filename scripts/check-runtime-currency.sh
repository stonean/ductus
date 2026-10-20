#!/usr/bin/env bash
# Report whether the runtime is current, and fail when it is not.
#
# Nothing else keeps it current. Before this check, Cargo.lock had drifted 121
# packages behind, two direct dependencies sat a major version back, the
# pinned toolchain was two releases old, and four rustls advisories were found
# only because someone ran `cargo audit` by hand — the push workflow runs it
# only when a commit touches runtime/. Each of those is a chore that depends on
# someone remembering it, so this asks all five questions on a schedule
# (.github/workflows/runtime-currency.yml) and from a contributor's shell:
#
#   1. lockfile     — `cargo update --dry-run` would change Cargo.lock.
#   2. majors       — a direct dependency has a newer, semver-incompatible
#                     release (moving it is a Cargo.toml and code change).
#   3. toolchain    — runtime/rust-toolchain.toml pins an older channel than
#                     the latest stable Rust.
#   4. advisories   — `cargo audit` reports a vulnerability (runtime/audit.toml
#                     still governs what is ignored).
#   5. pins         — a CI input the lockfile does not cover has fallen behind
#                     what its upstream publishes, or is not pinned at all
#                     (BE-DEPS-006; see the section below).
#
# A compatible update held back only by the declared `rust-version` (MSRV) is
# reported, never failed: the MSRV is a deliberate floor, raised on purpose.
#
# Exit status: 1 when any check found the runtime behind; otherwise 2 when any
# check could not run (no network, no cargo-audit), so an unexamined
# subject never reads as current; 0 only when every check ran and found nothing.
#
# Usage: scripts/check-runtime-currency.sh [check ...]   (from anywhere in the
# repo). With no arguments every check runs; naming checks (lockfile, majors,
# toolchain, advisories, pins) runs only those. lockfile and majors read one
# dry run, so naming either runs both.

set -uo pipefail

checks="lockfile majors toolchain advisories pins"
for arg in "$@"; do
  case " $checks " in
    *" $arg "*) ;;
    *)
      printf 'unknown check: %s (expected one of: %s)\n' "$arg" "$checks" >&2
      exit 2
      ;;
  esac
done
selected=" $* "
# True when the named check is to run: every check when none was named.
want() { [ "$selected" = "  " ] || case "$selected" in *" $1 "*) true ;; *) false ;; esac; }

root=$(git rev-parse --show-toplevel)
runtime="$root/runtime"
behind=0
unexamined=0

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

report() { printf '%s\n' "$*"; }

# Every lookup this script makes itself is bounded (BE-TIMEOUT-001); cargo's
# own requests in checks 1, 2 and 4 keep cargo's defaults. A connection gets
# lookup_connect_seconds and a whole request lookup_max_seconds, after which
# the check reports what it could not read rather than waiting on it.
lookup_connect_seconds=10
lookup_max_seconds=60
fetch() { curl -fsSL --connect-timeout "$lookup_connect_seconds" --max-time "$lookup_max_seconds" "$@"; }
# True when version $1 sorts strictly before version $2.
ver_lt() { [ "$1" != "$2" ] && [ "$(printf '%s\n%s\n' "$1" "$2" | sort -V | head -1)" = "$1" ]; }

# --- 1 + 2: lockfile and majors, from one verbose dry run --------------------
if ! want lockfile && ! want majors; then
  :
elif (cd "$runtime" && cargo update --dry-run --verbose) >"$tmp/update.out" 2>&1; then
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
if ! want toolchain; then
  :
elif fetch https://static.rust-lang.org/dist/channel-rust-stable.toml -o "$tmp/channel.toml"; then
  stable=$(awk '/^\[pkg\.rust\]$/{f=1} f && /^version = /{print; exit}' "$tmp/channel.toml" |
    sed -E 's/^version = "([0-9]+\.[0-9]+\.[0-9]+).*/\1/')
  if [ -z "$pinned" ] || [ -z "$stable" ]; then
    unexamined=1
    report "toolchain: NOT EXAMINED — pinned '${pinned:-?}', latest stable '${stable:-?}'"
  elif ver_lt "$pinned" "$stable"; then
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
if ! want advisories; then
  :
elif (cd "$runtime" && cargo audit --version) >/dev/null 2>&1; then
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

# --- 5: pins ------------------------------------------------------------------
# BE-DEPS-006 pins every CI input the lockfile does not cover: each action to
# a full commit SHA with its release tag in a same-line comment, each installed
# tool to an exact version. A pin stays current only if something compares it
# with upstream, and this is that comparison (022 task 3). An action pin is
# current when its tag is the action's latest release and still names the
# pinned commit; a tool pin is current when its version is the latest its
# registry reports (crates.io's newest stable version, PyPI's latest, or this
# repository's latest release for the adopter CI template's runtime, whose
# sha256 must also be the one that release published); and
# every `toolchain:` input must equal rust-toolchain.toml's channel, which
# check 3 holds against stable Rust. A reference with no pin at all is
# reported too: there is nothing to compare, and the build runs whatever its
# upstream points at that day.
#
# It reads the forms this repository's workflows use: `uses:`, taiki-e's
# `tool:`, `cargo install`, pip requirements, `toolchain:` and the template's
# DUCTUS_* pair. A download written in any other form is not seen.
#
# It reports and never edits. A red run names each pin and where it is, and
# the bump lands as its own commit like any other fix this script names.
ua="ductus check-runtime-currency (https://github.com/stonean/ductus)"
pins_behind=""
pins_unexamined=""
note_behind() { pins_behind+="  $*"$'\n'; }
note_unexamined() { pins_unexamined+="  $*"$'\n'; }
# The value of a `key: value` line, without quotes.
scalar_of() { printf '%s\n' "$2" | sed -E "s/^[[:space:]]*$1:[[:space:]]*['\"]?([^'\"[:space:]]*).*/\1/"; }
# "N reference(s), first at file:line" over the rows of tsv $1 whose first
# three fields are $2, $3 and $4; each row's fourth field is where it was read.
refs_summary() {
  awk -F'\t' -v a="$2" -v b="$3" -v c="$4" '
    $1 == a && $2 == b && $3 == c { n++; if (n == 1) first = $4 }
    END { printf "%d reference(s), first at %s", n, first }' "$1"
}

# Look up the newest stable version a registry publishes: crates.io or PyPI.
# Prints nothing when the lookup fails.
latest_crate() {
  fetch -A "$ua" "https://crates.io/api/v1/crates/$1" 2>/dev/null |
    jq -r '.crate.max_stable_version // empty' 2>/dev/null
}
latest_pypi() {
  fetch "https://pypi.org/pypi/$1/json" 2>/dev/null | jq -r '.info.version // empty' 2>/dev/null
}
# The GitHub REST API, through the same bounded fetch. GH_TOKEN, which
# runtime-currency.yml passes, raises the anonymous rate limit.
github_api() {
  local args=(-H 'Accept: application/vnd.github+json')
  if [ -n "${GH_TOKEN:-}" ]; then
    # From a file, so the token never appears in a process's arguments.
    if [ ! -e "$tmp/github-auth-header" ]; then
      (umask 077 && printf 'Authorization: Bearer %s\n' "$GH_TOKEN" >"$tmp/github-auth-header")
    fi
    args+=(-H "@$tmp/github-auth-header")
  fi
  fetch "${args[@]}" "https://api.github.com/$1"
}
# A repository's latest release tag, looked up once however many pins name it.
latest_release_of() {
  cache="$tmp/latest-release-$(printf '%s' "$1" | tr '/' '_')"
  if [ ! -e "$cache" ]; then
    github_api "repos/$1/releases/latest" 2>/dev/null | jq -r '.tag_name // empty' >"$cache" 2>/dev/null || : >"$cache"
  fi
  cat "$cache"
}
# This repository's own latest release, which the adopter CI template's
# install step downloads by version.
latest_ductus_release() { latest_release_of stonean/ductus | sed 's/^ductus-v//'; }

# Compare one tool pin with its registry. $1 kind, $2 name, $3 pinned version,
# $4 where it is pinned.
check_tool() {
  case "$1" in
    crate) latest=$(latest_crate "$2") ;;
    pypi) latest=$(latest_pypi "$2") ;;
    ductus-release) latest=$(latest_ductus_release) ;;
  esac
  if [ -z "$latest" ]; then
    note_unexamined "$2 $3 ($4): could not read the latest version from its registry"
  elif ver_lt "$3" "$latest"; then
    note_behind "$2 pins $3, latest is $latest ($4)"
  fi
}

if want pins; then
  pin_files=()
  while IFS= read -r f; do
    [ -n "$f" ] && pin_files+=("$f")
  done < <(cd "$root" && git ls-files -- '.github/workflows/*.yml' '.github/workflows/*.yaml' \
    'framework/templates/ci/*.yml' 'framework/templates/ci/*.yaml')

  if [ "${#pin_files[@]}" -eq 0 ]; then
    unexamined=1
    report "pins: NOT EXAMINED — no tracked workflow or CI template matched, so the enumeration is broken, not the pins"
  else
    for list in actions tools release-versions release-hashes; do : >"$tmp/$list.tsv"; done
    # Only the lines a branch below can match, as `file:line:text`, comments
    # dropped: one grep rather than a pass of subprocesses over every line.
    candidates='^[[:space:]]*(-[[:space:]]+)?(uses|tool|DUCTUS_VERSION|DUCTUS_SHA256|toolchain):[[:space:]]'
    candidates+='|(^|[^[:alnum:]_-])(cargo|pip3?) install[[:space:]]|==[0-9][^[:space:]]*[[:space:]]+--hash=sha256:'
    (cd "$root" && grep -H -n -E "$candidates" "${pin_files[@]}" |
      grep -v -E '^[^:]+:[0-9]+:[[:space:]]*#') >"$tmp/lines.txt"

    while IFS= read -r hit; do
      rest=${hit#*:}
      where="${hit%%:*}:${rest%%:*}"
      text=${rest#*:}

      # Actions: `uses: owner/repo[/path]@ref # tag`. A local reusable
      # workflow (`./…`) is this repository's own file and has no upstream.
      if printf '%s\n' "$text" | grep -qE '^[[:space:]]*(-[[:space:]]+)?uses:[[:space:]]'; then
        ref=$(printf '%s\n' "$text" | sed -E 's/^[[:space:]]*(-[[:space:]]+)?uses:[[:space:]]+([^[:space:]]+).*/\2/')
        case "$ref" in ./*) continue ;; esac
        sha=${ref##*@}
        repo=$(printf '%s\n' "${ref%@*}" | cut -d/ -f1-2)
        tag=$(printf '%s\n' "$text" | sed -nE 's/.*[[:space:]]#[[:space:]]*([^[:space:]]+)[[:space:]]*$/\1/p')
        if ! printf '%s\n' "$sha" | grep -qE '^[0-9a-f]{40}$'; then
          note_behind "$ref is not pinned to a commit SHA ($where)"
        elif [ -z "$tag" ]; then
          note_behind "$repo@$sha has no release tag in a comment to compare ($where)"
        else
          printf '%s\t%s\t%s\t%s\n' "$repo" "$sha" "$tag" "$where" >>"$tmp/actions.tsv"
        fi
        continue
      fi

      # taiki-e/install-action's `tool:` input: each tool as `name@version`.
      if printf '%s\n' "$text" | grep -qE '^[[:space:]]*tool:[[:space:]]'; then
        for item in $(printf '%s\n' "$text" | sed -E 's/^[[:space:]]*tool:[[:space:]]*//' | tr ',' ' '); do
          case "$item" in
            *@*) printf 'crate\t%s\t%s\t%s\n' "${item%@*}" "${item#*@}" "$where" >>"$tmp/tools.tsv" ;;
            *) note_behind "tool $item has no pinned version ($where)" ;;
          esac
        done
        continue
      fi

      # `cargo install` must name an exact version; cargo verifies the crate
      # against the registry's checksum, and a published version never changes.
      if printf '%s\n' "$text" | grep -qE '(^|[^[:alnum:]_-])cargo install[[:space:]]'; then
        name=$(printf '%s\n' "$text" |
          sed -E 's/.*cargo install[[:space:]]+((--[a-z-]+[[:space:]]+)*)([A-Za-z0-9_-]+).*/\3/')
        version=$(printf '%s\n' "$text" | sed -nE 's/.*[[:space:]]--version[[:space:]]+=?([0-9][^[:space:]]*).*/\1/p')
        if [ -z "$version" ]; then
          note_behind "cargo install $name has no pinned version ($where)"
        else
          printf 'crate\t%s\t%s\t%s\n' "$name" "$version" "$where" >>"$tmp/tools.tsv"
        fi
        continue
      fi

      # pip installs only in hash-checking mode, from requirements that pin
      # `name==version --hash=sha256:…`.
      if printf '%s\n' "$text" | grep -qE '(^|[^[:alnum:]_-])pip3? install[[:space:]]'; then
        if ! printf '%s\n' "$text" | grep -q -- '--require-hashes'; then
          note_behind "pip install without --require-hashes ($where)"
        fi
        continue
      fi
      if printf '%s\n' "$text" | grep -qE '[A-Za-z0-9_.-]+==[0-9][^[:space:]]*[[:space:]]+--hash=sha256:[0-9a-f]{64}'; then
        req=$(printf '%s\n' "$text" | grep -oE '[A-Za-z0-9_.-]+==[0-9][^[:space:]]*[[:space:]]+--hash' | head -1)
        req=${req%%[[:space:]]*}
        printf 'pypi\t%s\t%s\t%s\n' "${req%%==*}" "${req#*==}" "$where" >>"$tmp/tools.tsv"
        continue
      fi

      # The adopter CI template's install step pins a ductus release by
      # version and by its x86_64 Linux archive's sha256.
      if printf '%s\n' "$text" | grep -qE '^[[:space:]]*DUCTUS_VERSION:[[:space:]]'; then
        version=$(scalar_of DUCTUS_VERSION "$text")
        printf 'ductus-release\tductus\t%s\t%s\n' "$version" "$where" >>"$tmp/tools.tsv"
        printf '%s\t%s\t%s\n' "${where%%:*}" "$version" "$where" >>"$tmp/release-versions.tsv"
        continue
      fi
      if printf '%s\n' "$text" | grep -qE '^[[:space:]]*DUCTUS_SHA256:[[:space:]]'; then
        hash=$(scalar_of DUCTUS_SHA256 "$text")
        printf '%s\t%s\n' "${where%%:*}" "$hash" >>"$tmp/release-hashes.tsv"
        continue
      fi

      # dtolnay/rust-toolchain's `toolchain:` input moves with the toolchain file.
      if printf '%s\n' "$text" | grep -qE '^[[:space:]]*toolchain:[[:space:]]'; then
        version=$(scalar_of toolchain "$text")
        printf 'toolchain\trust\t%s\t%s\n' "$version" "$where" >>"$tmp/tools.tsv"
        continue
      fi
    done <"$tmp/lines.txt"

    # Each pinned ductus release needs its hash beside it, and the hash must
    # be the one the release published, or the install step fails on a hash
    # nobody can satisfy.
    while IFS=$'\t' read -r file version where; do
      hash=$(awk -F'\t' -v f="$file" '$1 == f { print $2; exit }' "$tmp/release-hashes.tsv")
      if [ -z "$hash" ]; then
        note_behind "ductus $version has no DUCTUS_SHA256 beside it ($where)"
        continue
      fi
      published=$(fetch "https://github.com/stonean/ductus/releases/download/ductus-v$version/ductus-x86_64-unknown-linux-gnu.tar.gz.sha256" 2>/dev/null | awk '{ print $1; exit }')
      if [ -z "$published" ]; then
        note_unexamined "ductus $version ($where): could not read the release's published sha256"
      elif [ "$published" != "$hash" ]; then
        note_behind "DUCTUS_SHA256 $hash is not the sha256 ductus $version published, $published ($where)"
      fi
    done <"$tmp/release-versions.tsv"

    # One lookup per distinct pin, however many lines repeat it.
    action_pins=0
    while IFS=$'\t' read -r repo sha tag; do
      action_pins=$((action_pins + 1))
      at=$(refs_summary "$tmp/actions.tsv" "$repo" "$sha" "$tag")
      commit=$(github_api "repos/$repo/commits/$tag" 2>/dev/null | jq -r '.sha // empty' 2>/dev/null)
      latest=$(latest_release_of "$repo")
      if [ -z "$commit" ] || [ -z "$latest" ]; then
        note_unexamined "$repo $tag ($at): could not read the tag's commit or the latest release from GitHub"
      else
        if [ "$commit" != "$sha" ]; then
          note_behind "$repo pins $sha as $tag, but $tag now names $commit ($at)"
        fi
        if ver_lt "${tag#v}" "${latest#v}"; then
          note_behind "$repo pins $tag, latest release is $latest ($at)"
        fi
      fi
    done < <(cut -f1-3 "$tmp/actions.tsv" | sort -u)

    tool_pins=0
    while IFS=$'\t' read -r kind name version; do
      tool_pins=$((tool_pins + 1))
      at=$(refs_summary "$tmp/tools.tsv" "$kind" "$name" "$version")
      if [ "$kind" = toolchain ]; then
        if [ "$version" != "$pinned" ]; then
          note_behind "toolchain: $version differs from runtime/rust-toolchain.toml's $pinned ($at)"
        fi
      else
        check_tool "$kind" "$name" "$version" "$at"
      fi
    done < <(cut -f1-3 "$tmp/tools.tsv" | sort -u)

    scope="$action_pins action pin(s) and $tool_pins tool pin(s) across ${#pin_files[@]} file(s)"
    if [ -n "$pins_behind" ]; then
      behind=1
      report "pins: behind — $(printf '%s' "$pins_behind" | grep -c .) to update, from $scope:"
      printf '%s' "$pins_behind"
    fi
    if [ -n "$pins_unexamined" ]; then
      unexamined=1
      report "pins: NOT EXAMINED — $(printf '%s' "$pins_unexamined" | grep -c .) of $scope:"
      printf '%s' "$pins_unexamined"
    fi
    if [ -z "$pins_behind" ] && [ -z "$pins_unexamined" ]; then
      report "pins: current ($scope)"
    fi
  fi
fi

if [ "$behind" -eq 1 ]; then
  exit 1
elif [ "$unexamined" -eq 1 ]; then
  exit 2
fi
exit 0
