#!/usr/bin/env bash
# Measure every primitive's result size and every tool's loaded schema size on
# a generated fixture corpus, with no model (spec 080, AC14, AC23).
#
# Runs the ignored `result_sizes` test in runtime/tests/result_sizes.rs, which
# builds two corpora (runtime/tests/common/corpus.rs), calls every name in
# PRIMITIVE_REGISTRY, and each argument mode that widens a read, through the
# built binary's MCP server on a fresh copy of each, and records the
# characters of each result's text: on both corpora at a read size past every
# result, which is what the growth test reads, and on the large one at the
# default read size, which is what a host reads. A name the fixture cannot
# exercise gets a "not measured" row with its reason. The report opens
# with the commit it was measured at, each uncommitted path under runtime/
# (the binary is built from the work tree), and the UTC time, and goes to
# stdout, or to the file given as the first argument.
#
# On demand only: no workflow runs it, and `cargo test` skips the test.
#
# Usage: scripts/measure-result-sizes.sh [report-file]

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

if [ "$#" -gt 1 ]; then
  echo "usage: $0 [report-file]" >&2
  exit 2
fi

body="$(mktemp -t measure-result-sizes-XXXXXX)"
trap 'rm -f "$body"' EXIT

(cd "$ROOT/runtime" && DUCTUS_RESULT_SIZES_OUT="$body" \
  cargo test --test result_sizes -- --ignored --nocapture --exact result_sizes >&2)

sha="$(git -C "$ROOT" rev-parse HEAD)"
tree="a clean runtime/"
changed="$(git -C "$ROOT" status --porcelain -- runtime | cut -c4- | paste -sd, - | sed 's/,/, /g')"
if [ -n "$changed" ]; then
  tree="these uncommitted under runtime/: $changed"
fi
stamp="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

report() {
  printf "Measured by \`scripts/measure-result-sizes.sh\` at %s, with %s, at %s.\n\n" "$sha" "$tree" "$stamp"
  cat "$body"
}

if [ "$#" -eq 1 ]; then
  report > "$1"
else
  report
fi
