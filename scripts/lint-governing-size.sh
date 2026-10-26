#!/usr/bin/env bash
# Hold the governing documents to their size ceilings.
#
# Every lesson used to become a permanent rule and nothing was retired, so
# AGENTS.md grew to 283,875 bytes and the constitution to 119,262 — six reads
# and three at the 50,000-byte read size an agent can take in one call. A
# ceiling checked on every commit is the exit: a lesson added past it retires,
# merges or relocates another in the same change, and nobody has to remember
# the budget.
#
# Each file is measured as it sits in the git index — the staged blob in the
# pre-commit hook, the checkout in CI — so an unstaged edit neither fails nor
# passes a commit. A file missing from the index, one the index holds but
# cannot be read, and a ceiling that is not a whole number of bytes each fail
# with a message of their own: a check that cannot measure must never read as
# one that passed.
#
# Source of truth: specs/072-governing-documents-stay-within-one-read/spec.md
# Consumed by: .githooks/pre-commit, .github/workflows/framework-checks.yml

set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

for arg in "$@"; do
  case "$arg" in
    -h|--help)
      sed -n '2,18p' "$0" | sed 's/^# \{0,1\}//'
      echo
      echo "Usage: $(basename "$0")"
      echo "  Exits 0 when every governed file is at or under its ceiling."
      echo "  Exits 1 when any file is over, missing from the index, or unreadable,"
      echo "  or when a ceiling is not a whole number of bytes."
      exit 0
      ;;
    *) echo "Unknown argument: $arg" >&2; exit 2 ;;
  esac
done

# path | ceiling in bytes | fix named when the file is over
#
# Raising a ceiling is an owner decision recorded in spec 072 through its
# back-edge, never a choice made at commit time.
#
# AGENTS.md: 50,000 is DEFAULT_READ_SIZE_BYTES in
# runtime/src/primitives/check_artifact_size.rs — the read size spec 063
# applies to spec artifacts — and scripts/tests/test-lint-governing-size.sh
# fails when the two disagree. It is the default, not `[artifacts]
# read-size-bytes`: this repository sets no such key.
#
# framework/constitution.md: the size 050-constitution's cut to one read
# reached (scenario the-constitution-fits-one-read). Keeping true the passages
# other specs quote held it above the 50,000-byte read size, so whether the
# read size or this figure is the target is that scenario's open question for
# the owner. A rule only one command applies belongs in that command's
# framework/reference/ file.
CEILINGS="AGENTS.md|50000|retire, merge, or move an entry to the check that enforces it (spec 072)
framework/constitution.md|53813|cut it, not the ceiling — move a rule only one command applies to its framework/reference/ file, or tighten one (spec 050)"

errors=0
while IFS='|' read -r path ceiling fix; do
  [ -n "$path" ] || continue
  case "$ceiling" in
    '' | *[!0-9]*)
      echo "$path: its ceiling '$ceiling' is not a whole number of bytes — correct the CEILINGS table in scripts/lint-governing-size.sh"
      errors=$((errors + 1))
      continue
      ;;
  esac
  if ! git -C "$ROOT" ls-files --error-unmatch -- "$path" > /dev/null 2>&1; then
    echo "$path: not in the index, so its size cannot be measured"
    errors=$((errors + 1))
    continue
  fi
  if ! size="$(git -C "$ROOT" cat-file -s ":$path" 2> /dev/null)"; then
    echo "$path: could not be read from the index"
    errors=$((errors + 1))
    continue
  fi
  if [ "$size" -gt "$ceiling" ]; then
    echo "$path: $size bytes, over its $ceiling-byte ceiling — $fix"
    errors=$((errors + 1))
  fi
done <<EOF
$CEILINGS
EOF

if [ "$errors" -gt 0 ]; then
  exit 1
fi
exit 0
