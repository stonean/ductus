#!/usr/bin/env bash
# List what a change may have left stale: each literal its diff removes, with
# every place the live-artifact set still holds it.
#
# A change edits one statement and leaves the ones that depend on it. 022's
# review fix rounds of 2026-11-17 found these slips one re-read at a time:
#   - a fixture copying a contract string the change narrowed;
#   - a figure another landing had moved;
#   - a header naming a count the change made untrue.
# docs/developing-ductus.md §Landing a runtime change, step 3, runs this before
# a change commits and before a fix round's brief-re-read. Reading the change's
# enumerations and re-measuring its figures stay the operator's: a word like
# "two" is no literal this can search for.
#
# Usage: scripts/stale-literals.sh BASE
#
# BASE is a commit. The change is the diff from BASE to the working tree,
# staged and unstaged alike. A literal is either of these, found on a line the
# diff removes:
#   - a backticked span of 2 to 80 characters;
#   - a number of three or more digits, comma-grouped or not, standing alone as
#     a token (a four-digit year is left out, and so are the digits inside a sha).
# Each literal is searched as a fixed string over the live-artifact set that
# AGENTS.md names between its audit:sweep-targets markers. This script reads
# the list there rather than restating it.
#
# A literal counts as removed only when the diff removes more copies of it than
# it adds back, so one an edited line keeps on both sides is not listed.
#
# Output:
#   - for each removed literal still found, `== <literal>: <n> hit(s)` and its
#     hits;
#   - a last line counting the removed literals examined and those still found.
#
# Exit status:
#   - 0 when the search ran, whatever it found. A hit is a worklist item, to be
#     updated or named as correct.
#   - 2 when it cannot run: no BASE, a BASE that does not resolve, an
#     AGENTS.md with no sweep-targets markers or no target between them, or a
#     search that errors. So an empty result never stands in for a search that
#     did not happen.

set -euo pipefail

if [ "$#" -ne 1 ]; then
  printf 'usage: %s BASE\n' "$0" >&2
  exit 2
fi
base="$1"

if ! root="$(git rev-parse --show-toplevel 2>/dev/null)"; then
  printf 'stale-literals: not inside a git work tree\n' >&2
  exit 2
fi
cd "$root"

if ! git rev-parse --verify --quiet "${base}^{commit}" >/dev/null; then
  printf 'stale-literals: %s does not resolve to a commit\n' "$base" >&2
  exit 2
fi

# The live-artifact set, read from AGENTS.md: the backticked entries between
# the markers, a parenthesized "including ..." aside left out, `NNN` read as
# three digits, and a trailing slash dropped so the entry is a path prefix.
targets_text=""
if [ -f AGENTS.md ]; then
  targets_text="$(sed -n 's/.*<!-- audit:sweep-targets:begin -->\(.*\)<!-- audit:sweep-targets:end -->.*/\1/p' AGENTS.md)"
fi
targets=()
# shellcheck disable=SC2016
while IFS= read -r entry; do
  [ -n "$entry" ] || continue
  entry="${entry%/}"
  entry="$(printf '%s' "$entry" | sed 's/NNN/[0-9][0-9][0-9]/g')"
  targets+=("$entry")
done < <(printf '%s\n' "$targets_text" | sed 's/([^)]*)//g' | grep -oE '`[^`]+`' | tr -d '`' || true)
if [ "${#targets[@]}" -eq 0 ]; then
  printf 'stale-literals: AGENTS.md names no live-artifact set between its audit:sweep-targets markers\n' >&2
  exit 2
fi

diff_text="$(git diff -U0 --no-color --no-ext-diff "$base" --)"
removed="$(printf '%s\n' "$diff_text" | grep -E '^-' | grep -vE '^--- ' || true)"
added="$(printf '%s\n' "$diff_text" | grep -E '^\+' | grep -vE '^\+\+\+ ' || true)"

# The literals on one side of the diff, one per occurrence, so a literal an
# edited line keeps (it is on both sides) is told from one the diff removes.
# The backticks in these patterns are literal characters, never expansions.
side_literals() {
  # shellcheck disable=SC2016
  printf '%s\n' "$1" | grep -oE '`[^`]{2,80}`' | sed 's/^`//; s/`$//' || true
  printf '%s\n' "$1" | tr -c '0-9A-Za-z,\n' ' ' | tr ' ' '\n' | sed 's/^,*//; s/,*$//' \
    | grep -E '^([0-9]{1,3}(,[0-9]{3})+|[0-9]{3,})$' | grep -vE '^(19|20)[0-9]{2}$' || true
}
removed_literals="$(side_literals "$removed")"
added_literals="$(side_literals "$added")"
occurrences() {
  printf '%s\n' "$2" | grep -cxF -e "$1" || true
}

examined=0
with_hits=0
while IFS= read -r lit; do
  [ -n "$lit" ] || continue
  # Removed only when the diff takes away more copies than it puts back.
  if [ "$(occurrences "$lit" "$removed_literals")" -le "$(occurrences "$lit" "$added_literals")" ]; then
    continue
  fi
  examined=$((examined + 1))
  set +e
  hits="$(git grep -nF -e "$lit" -- "${targets[@]}")"
  code=$?
  set -e
  if [ "$code" -eq 1 ]; then
    continue
  fi
  if [ "$code" -ne 0 ]; then
    printf 'stale-literals: the search for %s failed (git grep exit %s)\n' "$lit" "$code" >&2
    exit 2
  fi
  with_hits=$((with_hits + 1))
  count="$(printf '%s\n' "$hits" | wc -l | tr -d ' ')"
  printf '== %s: %s hit(s)\n' "$lit" "$count"
  printf '%s\n' "$hits"
done <<EOF
$(printf '%s\n' "$removed_literals" | sort -u)
EOF

printf 'stale-literals: examined %s literal(s) removed since %s over %s target(s); %s still found\n' \
  "$examined" "$base" "${#targets[@]}" "$with_hits"
