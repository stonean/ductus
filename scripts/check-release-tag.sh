#!/usr/bin/env bash
# Verify that the tag a release runs for names the repo-root `version` file.
#
# `version`, runtime/Cargo.toml and the newest runtime/CHANGELOG.md heading are
# bumped in one commit, and the self-audit's Family 20 holds those three in
# agreement; the tag is pushed after that commit, so no family can see it.
# This is where the tag is checked: the release workflow's `audit` job runs it,
# and `publish`, the irreversible crates.io upload, needs that job, so a tag
# that names another version stops the release before anything is published
# (spec 048, AC15).
#
# It also refuses a tag while an entry (a `- ` item or a `### ` subsection)
# sits above runtime/CHANGELOG.md's newest `## [X.Y.Z]` heading, whatever its
# section is called (`## [Unreleased]`, `## Unreleased` or any other): the
# release commit gives that section its version, and Family 20 reads only the
# newest versioned heading, so entries left above it would ship unannounced
# (spec 008, task 43 and the scenario
# the-tag-check-reads-every-entry-above-the-newest-version). Prose alone above
# it, the file's own introduction included, passes.
#
# Usage: scripts/check-release-tag.sh <ref-name>
#   <ref-name> is the pushed tag, e.g. "$GITHUB_REF_NAME" (`ductus-v0.60.0`).
#   Exits 0 when it is `ductus-v` followed by `version`'s one line and no entry
#   sits above the changelog's newest version heading.
#   Exits 1 when it names anything else, or an entry sits there (printed).
#   Exits 2 when no ref is given, `version` is missing or empty, or
#   runtime/CHANGELOG.md is missing.
#
# Consumed by: .github/workflows/runtime-release.yml (job `audit`)
# Tested by: scripts/tests/test-check-release-tag.sh

set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

case "${1:-}" in
  -h|--help)
    sed -n '2,27p' "$0" | sed 's/^# \{0,1\}//'
    exit 0
    ;;
  '')
    echo "Usage: $(basename "$0") <ref-name>" >&2
    exit 2
    ;;
esac
if [ "$#" -ne 1 ]; then
  echo "Usage: $(basename "$0") <ref-name>" >&2
  exit 2
fi
ref="$1"

if [ ! -f "$ROOT/version" ]; then
  echo "version: not found — the tag cannot be compared with anything" >&2
  exit 2
fi
version="$(tr -d '[:space:]' < "$ROOT/version")"
if [ -z "$version" ]; then
  echo "version: empty — the tag cannot be compared with anything" >&2
  exit 2
fi

expected="ductus-v${version}"
if [ "$ref" != "$expected" ]; then
  echo "tag '${ref}' does not name version ${version}: expected '${expected}'. Nothing has been published; delete the tag, and push '${expected}' once version, runtime/Cargo.toml and runtime/CHANGELOG.md agree."
  exit 1
fi
changelog="$ROOT/runtime/CHANGELOG.md"
if [ ! -f "$changelog" ]; then
  echo "runtime/CHANGELOG.md: not found — whether the release versioned every pending entry cannot be checked" >&2
  exit 2
fi
# The newest version heading is the first, matched as Family 20 matches it
# (scripts/audit/version-agreement.sh); every entry above it is pending.
newest="$(awk '/^## \[[0-9]+\.[0-9]+\.[0-9]+\]/ { print; exit }' "$changelog")"
unreleased="$(awk '
  /^## \[[0-9]+\.[0-9]+\.[0-9]+\]/ { exit }
  /^- / || /^### / { n++ }
  END { print n + 0 }
' "$changelog")"
if [ "$unreleased" -ne 0 ]; then
  if [ -n "$newest" ]; then
    where="above its newest version heading '${newest}'"
  else
    where="in it, and it has no version heading"
  fi
  echo "runtime/CHANGELOG.md: ${unreleased} entry line(s) sit ${where}. Nothing has been published; delete the tag, give those entries version ${version} and its date in the release commit, and push '${expected}' again."
  exit 1
fi
echo "tag '${ref}' names version ${version}"
