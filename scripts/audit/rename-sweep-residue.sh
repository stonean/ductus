#!/usr/bin/env bash
# scripts/audit/rename-sweep-residue.sh — Family 24 of /audit.
#
# 049 renamed the project with a word-boundary `govern` -> `ductus`
# substitution. `govern` was in use as both a noun (the project) and an
# ordinary English verb, and a word-boundary sweep cannot tell them apart: it
# renamed the noun correctly and replaced the verb with a proper noun, leaving
# sentences like "a class of behavior the framework should ductus at the rules
# tier".
#
# security-frontend.md shows the mechanism inside one sentence — "The other
# `FE-DEPS` rules ductus what code is loaded ... none governs what a dependency
# does" — where the inflected form survived only because it never matched the
# word boundary.
#
# Eight sites survived. Three ship to adopters, and one of those is
# framework/templates/project/agents.md, a `create`-strategy file: the broken
# sentence is written into a new adopter's AGENTS.md once and corrected by no
# later run. All 23 existing families were green throughout; the residue was
# found by an ad-hoc grep during an unrelated review.
#
# DETECTION. Two constructions, both drawn from closed word classes, so this is
# exact rather than heuristic:
#
#   1. A modal + the project name. A modal is always followed by a bare
#      infinitive and the project name is a proper noun, so the pair cannot be
#      correct under any phrasing.
#   2. The project name + a demonstrative or wh-word. A proper noun does not
#      take one.
#
# MEASURED. The union reports 8 findings at the commit before the repair —
# exactly the 8 real sites, no others — and 0 after it.
#
# DELIBERATE EXCLUSIONS. `the` is absent from the second list: "with `PATH`
# stripped of ductus the same commit succeeds" is correct prose. `to ductus`
# and `that ductus` are absent for the same reason — "a change to ductus" and
# "the version that ductus pins" are ordinary. Each would trade a real finding
# for a standing false positive.
#
# A degenerate scan — no files examined — is a finding, never a pass. A
# corpus-wide grep that matches nothing is otherwise indistinguishable from a
# clean corpus. See AGENTS.md §Design Principles.
#
# Bash 3.2 compatible (macOS system bash); POSIX grep only, no awk extensions
# (the portability trap that made Family 7 dead on macOS).

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family sweep-residue

# The project name, as the sweep would have written it.
NAME="ductus"

# Closed-class English modals: always followed by a bare infinitive.
MODALS="should|must|shall|may|might|would|could|can"

# Demonstratives and wh-words: a proper noun does not take one.
FOLLOWERS="this|what|how|across|whether|these|those"

PATTERN="(^|[^[:alnum:]_])($MODALS) $NAME([^[:alnum:]_]|\$)|(^|[^[:alnum:]_])$NAME ($FOLLOWERS)([^[:alnum:]_]|\$)"

# --- Collect the corpus ---------------------------------------------------
#
# Tracked markdown only. Untracked drafts are deliberately invisible to the
# pipeline (§text-first-artifacts, tracked-specs rule), and runtime test
# fixtures mirror arbitrary adopter content rather than authored documentation
# — the same carve-out .markdownlint-cli2.jsonc makes.
files="$(git ls-files '*.md' 2>/dev/null | grep -v '^runtime/tests/')"
file_count="$(printf '%s\n' "$files" | grep -c '[^[:space:]]')"

if [ "$file_count" -eq 0 ]; then
  emit "repository" \
    "no tracked markdown files found — the residue scan examined nothing, which is not the same as finding nothing" \
    "run from a git checkout with tracked .md files; if the corpus moved, realign the file list in scripts/audit/rename-sweep-residue.sh"
  exit "$drift"
fi

# --- Scan -----------------------------------------------------------------
examined=0
while IFS= read -r f; do
  [ -z "$f" ] && continue
  [ -f "$f" ] || continue
  examined=$((examined + 1))
  # Strip code spans and quoted spans before matching. Prose that *documents*
  # this defect has to quote it — this family's own README entry and scenario
  # both do — and a quoted example is the one place the broken phrasing is
  # deliberate. The real residue is never quoted: it reads as ordinary prose,
  # which is exactly what made it survive three releases. sed rewrites in
  # place per line, so line numbers are preserved for the report.
  # shellcheck disable=SC2016  # the backticks are literal sed syntax for a code span, not a command substitution
  hits="$(sed -e 's/`[^`]*`//g' -e 's/"[^"]*"//g' -e 's/“[^”]*”//g' "$f" 2>/dev/null | grep -nEi "$PATTERN" 2>/dev/null)"
  while IFS= read -r hit; do
    [ -z "$hit" ] && continue
    lineno="${hit%%:*}"
    emit "$f:$lineno" \
      "the project name appears where a verb belongs — residue from a word-boundary rename sweep that replaced the verb \`govern\` with the project noun" \
      "restore the verb (\`govern\`) in this sentence; the repair changes no claim, so it is a mechanical edit under §spec-lifecycle and done specs stay done"
  done <<< "$hits"
done <<< "$files"

printf 'sweep-residue: %d markdown file%s examined for the project name in verb position\n' \
  "$examined" "$([ "$examined" -eq 1 ] && echo '' || echo s)" >&2

exit "$drift"
