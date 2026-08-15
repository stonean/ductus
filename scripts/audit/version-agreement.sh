#!/usr/bin/env bash
# scripts/audit/version-agreement.sh — Family 20 of /audit.
#
# The product has one version, recorded in more than one place. This family
# asserts the places agree.
#
# Spec 048 makes `/govern` acquire the runtime and pin it to the version the
# fetched framework revision declares, read from the repo-root `version`
# file. That pin is only as good as its agreement with what actually ships:
# if `version` says 0.28.0 while the runtime crate builds 0.27.2, every
# adopter downloads assets for a release that does not exist, and the failure
# surfaces as a halt during someone else's bootstrap rather than here.
#
# Three artifacts must carry the same SemVer, and all three are advanced by
# the same release commit:
#
#   20a  version                    — the pin /govern reads from the archive
#   20b  runtime/Cargo.toml         — what the crate builds and publishes as
#   20c  runtime/CHANGELOG.md       — the newest `## [X.Y.Z]` heading
#
# The release TAG is deliberately NOT compared. Per AGENTS.md the release
# order is bump + commit + push, then tag — so between those steps the three
# files legitimately name a version with no tag yet, and asserting the tag
# here would fail every release at exactly the moment the maintainer is
# mid-release. The tag is verified by the release pipeline itself, which runs
# after tagging and therefore can see it. Spec 048 records this split; it is
# a scope decision, not an oversight.
#
# Bash 3.2 compatible (macOS system bash).

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family version-agreement

VERSION_FILE="version"
CARGO_TOML="runtime/Cargo.toml"
CHANGELOG="runtime/CHANGELOG.md"

# 20a — the pin itself. Absent is a finding rather than a skip: /govern reads
# this file to decide what to download, so a missing pin is not "nothing to
# check", it is an unbootstrappable framework revision.
if [ ! -f "$VERSION_FILE" ]; then
  emit "$VERSION_FILE" "missing — /govern reads this file to resolve which runtime to acquire" \
    "create $VERSION_FILE containing one SemVer line matching runtime/Cargo.toml"
  exit "$drift"
fi

# Read the first line only, trimmed. A file with trailing content is still
# read for its first line, and the shape check below reports the rest.
pin=$(head -n 1 "$VERSION_FILE" | tr -d '[:space:]')

if ! printf '%s' "$pin" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
  emit "$VERSION_FILE" "value '$pin' is not a bare MAJOR.MINOR.PATCH SemVer" \
    "write exactly one line of the form 0.28.0 — no 'v' prefix, no pre-release suffix"
  exit "$drift"
fi

# More than one non-empty line means something else is being recorded here.
extra=$(grep -c '[^[:space:]]' "$VERSION_FILE")
if [ "$extra" -ne 1 ]; then
  emit "$VERSION_FILE" "contains $extra non-empty lines — the pin is a single SemVer line" \
    "reduce $VERSION_FILE to one line"
fi

# 20b — the crate version. Anchored to the [package] table's first `version =`
# so a dependency's version field can never be mistaken for the crate's.
if [ -f "$CARGO_TOML" ]; then
  cargo_version=$(awk '
    /^\[package\]/ { in_pkg = 1; next }
    /^\[/          { in_pkg = 0 }
    in_pkg && /^version[[:space:]]*=/ {
      gsub(/[^0-9.]/, "", $0); print; exit
    }
  ' "$CARGO_TOML")
  if [ -z "$cargo_version" ]; then
    emit "$CARGO_TOML" "no version field found under [package]" \
      "restore the crate version declaration"
  elif [ "$cargo_version" != "$pin" ]; then
    emit "$CARGO_TOML" "crate version '$cargo_version' disagrees with $VERSION_FILE '$pin'" \
      "bump both in the same commit — the pin names the runtime this framework revision requires"
  fi
else
  emit "$CARGO_TOML" "missing — cannot anchor the version-agreement check" \
    "restore the runtime crate or remove this audit family"
fi

# 20c — the newest CHANGELOG heading. `## [X.Y.Z] — date`, first match wins,
# matching the file's newest-first ordering.
if [ -f "$CHANGELOG" ]; then
  changelog_version=$(awk 'match($0, /^## \[[0-9]+\.[0-9]+\.[0-9]+\]/) {
    v = substr($0, RSTART + 4, RLENGTH - 5); print v; exit
  }' "$CHANGELOG")
  if [ -z "$changelog_version" ]; then
    emit "$CHANGELOG" "no '## [X.Y.Z]' release heading found" \
      "add a release section for $pin"
  elif [ "$changelog_version" != "$pin" ]; then
    emit "$CHANGELOG" "newest release heading '$changelog_version' disagrees with $VERSION_FILE '$pin'" \
      "add or correct the $pin section — a pin with no changelog entry ships undocumented"
  fi
else
  emit "$CHANGELOG" "missing — cannot verify the newest release heading" \
    "restore runtime/CHANGELOG.md"
fi

exit "$drift"
