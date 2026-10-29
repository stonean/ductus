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
# Usage: scripts/check-release-tag.sh <ref-name>
#   <ref-name> is the pushed tag, e.g. "$GITHUB_REF_NAME" (`ductus-v0.60.0`).
#   Exits 0 when it is `ductus-v` followed by `version`'s one line.
#   Exits 1 when it names anything else (the mismatch printed to stdout).
#   Exits 2 when no ref is given or `version` is missing or empty.
#
# Consumed by: .github/workflows/runtime-release.yml (job `audit`)
# Tested by: scripts/tests/test-check-release-tag.sh

set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

case "${1:-}" in
  -h|--help)
    sed -n '2,19p' "$0" | sed 's/^# \{0,1\}//'
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
echo "tag '${ref}' names version ${version}"
