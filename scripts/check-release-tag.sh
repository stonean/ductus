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
# It also refuses a tag while runtime/CHANGELOG.md's `## [Unreleased]` section
# holds an entry (a `- ` item or a `### ` subsection): the release commit gives
# that section its version, and Family 20 reads only the newest versioned
# heading, so entries left under `[Unreleased]` would ship unannounced (spec
# 008, task 43). A section holding only prose, or none, passes.
#
# Usage: scripts/check-release-tag.sh <ref-name>
#   <ref-name> is the pushed tag, e.g. "$GITHUB_REF_NAME" (`ductus-v0.60.0`).
#   Exits 0 when it is `ductus-v` followed by `version`'s one line and no entry
#   sits under `## [Unreleased]`.
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
  echo "runtime/CHANGELOG.md: not found — whether the release gave [Unreleased] its version cannot be checked" >&2
  exit 2
fi
unreleased="$(awk '
  /^## \[Unreleased\]/ { inside = 1; next }
  /^## / { inside = 0 }
  inside && (/^- / || /^### /) { n++ }
  END { print n + 0 }
' "$changelog")"
if [ "$unreleased" -ne 0 ]; then
  echo "runtime/CHANGELOG.md: ${unreleased} entry line(s) still sit under '## [Unreleased]'. Nothing has been published; delete the tag, give that section version ${version} and its date in the release commit, and push '${expected}' again."
  exit 1
fi
echo "tag '${ref}' names version ${version}"
