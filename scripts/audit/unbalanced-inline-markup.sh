#!/usr/bin/env bash
# scripts/audit/unbalanced-inline-markup.sh — Family 25 of /audit.
#
# Spec 050 rewrote 21 AGENTS.md entries to point at the constitution rather
# than restate it. Two of the 21 did not survive the rewrite: one had its bold
# title deleted rather than converted, leaving an orphan backtick and no title
# at all; the other kept its old body and had `** The rule is ...` appended,
# where the `**` is followed by a space and so never opens emphasis — it
# renders literally.
#
# Both are invisible to markdownlint. An unclosed backtick never becomes a code
# span, so MD038 does not apply, and a literal `**` is not a heading, list, or
# link. 050's review recorded clean and both stood until a reader noticed the
# rendered text.
#
# SCOPE, AND WHY IT IS TWO FILES. The check is per-line, which is exact for
# these two and only these two: every rule entry is a single unwrapped bullet
# (69 bullets, 0 continuation lines), so a span that opens on a line must close
# on it. It is deliberately NOT extended to the rest of the corpus, where prose
# wraps freely and a bold span legitimately crosses a line break — measured at
# 283 such lines, every one of them correct. Widening the scope would trade two
# real findings for 283 false ones.
#
# The scope is reported on stderr for the reason §design-principles gives: a
# clean exit here means "these two files are balanced", never "all markdown is
# balanced", and the two must not read alike.
#
# THE CONVENTION IS PART OF THE CONTRACT. A per-line check is only exact while
# the bullets stay unwrapped, so a continuation line is itself reported. The
# alternative — narrowing quietly once the subject drifts — is the failure this
# directory exists to prevent.
#
# MEASURED. 2 findings at the commit before the repair, exactly the two
# malformed entries; 0 after.
#
# Bash 3.2 compatible (macOS system bash); POSIX awk only, no GNU extensions
# (the portability trap that made Family 7 dead on macOS). awk emits
# tab-separated records and the shell renders them through `emit`, so the
# pipe-separated finding shape never appears in the awk.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family inline-markup

# The two files whose single-line-bullet convention makes a per-line balance
# check exact: this repo's own rules file, and the template that seeds an
# adopter's copy of it.
TARGETS=(AGENTS.md framework/templates/project/agents.md)

examined=0
for f in "${TARGETS[@]}"; do
  if [ ! -f "$f" ]; then
    emit "$f" \
      "target file is missing — the inline-markup check examined nothing for it, which is not the same as finding it balanced" \
      "restore $f, or realign the target list in scripts/audit/unbalanced-inline-markup.sh if the file was intentionally relocated"
    continue
  fi
  examined=$((examined + 1))

  while IFS="$(printf '\t')" read -r lineno message fix; do
    [ -z "$lineno" ] && continue
    emit "$f:$lineno" "$message" "$fix"
  done <<EOF
$(awk '
  # Fence toggling. A fence marker is itself three backticks, so it must be
  # consumed before any counting happens.
  /^[[:space:]]*(```|~~~)/ { fence = !fence; prev_bullet = 0; next }
  fence { next }

  {
    line = $0

    # Backticks must pair on the line. An odd count means a code span was
    # opened and never closed — which markdownlint does not see, because an
    # unclosed backtick never becomes a code span at all.
    tmp = line
    ticks = gsub(/`/, "", tmp)
    if (ticks % 2 == 1) {
      printf "%d\tan odd number of backticks (%d) — a code span is opened and never closed, so the rest of the line renders as literal text\tclose the span, or drop the stray backtick; markdownlint cannot see this because an unclosed backtick never becomes a code span\n", NR, ticks
    }

    # `**` markers must pair likewise. A `**` followed by whitespace never
    # opens emphasis and renders literally — the shape 050 left behind.
    tmp = line
    bolds = gsub(/\*\*/, "", tmp)
    if (bolds % 2 == 1) {
      printf "%d\tan odd number of `**` markers (%d) — one never opens or closes emphasis and renders literally\tclose the bold span, or drop the stray `**`; a `**` followed by a space cannot open emphasis\n", NR, bolds
    }

    # The per-line check is exact only while bullets stay unwrapped. Report a
    # continuation line rather than letting the scope drift out from under the
    # check.
    if (prev_bullet && $0 !~ /^[[:space:]]*$/ && $0 !~ /^[-*#>|]/ && $0 !~ /^[[:space:]]/ && $0 !~ /^<!--/) {
      printf "%d\ta bullet wraps onto this line, which breaks the single-line convention the per-line balance check depends on\tre-join the bullet onto one line, or widen this family to balance across the whole bullet before the convention lapses silently\n", NR
    }
    prev_bullet = ($0 ~ /^-[[:space:]]/)
  }
' "$f")
EOF
done

printf 'inline-markup: %d of %d target file%s examined (%s); corpus-wide balance NOT checked — prose wraps legitimately elsewhere\n' \
  "$examined" "${#TARGETS[@]}" \
  "$([ "${#TARGETS[@]}" -eq 1 ] && echo '' || echo s)" \
  "$(IFS=,; printf '%s' "${TARGETS[*]}")" >&2

exit "$drift"
