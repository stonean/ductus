#!/usr/bin/env bash
# scripts/audit/permission-entry-shape.sh — Family 32 of /audit.
#
# A canonical permission entry that scopes a FILE PATH must be written
# `Edit(path)`, never `Write(path)`. Claude Code matches file permissions
# against `Edit` rules only, and an `Edit` rule covers every file-editing
# tool — `Write` included. So a path-scoped `Write` entry grants nothing
# its `Edit` sibling has not already granted:
#
#     Edit(.ductus/session.toml)     <- grants the access
#     Write(.ductus/session.toml)    <- inert; warned about at startup
#
# Two such entries shipped in `configure/claude.md` §5 alongside the very
# `Edit(...)` entries that made them redundant, and reached every adopter
# that ran `/ductus:configure`. The host's own startup linter is what
# surfaced them — the same way it surfaced the seven wildcard-position
# entries Family 29 now guards.
#
# WHY THIS IS NOT FAMILY 29. The two invariants share a subject (the shape
# of an entry in a host's canonical set) and both were found by the host
# rather than by this suite, but they are opposite defects with opposite
# scopes:
#
#   - Family 29's entry grants too MUCH, so DENY is excluded by
#     construction — an over-broad deny refuses more rather than approving
#     more, and auditing both arrays under one rule would push a maintainer
#     to narrow the deny set into holes.
#   - This entry grants NOTHING, which is just as wrong on the deny side
#     and arguably worse: a `Write(path)` deny entry reads as a protected
#     path and protects nothing. So BOTH arrays are subjects here.
#
# Folding this into Family 29 would therefore have to relax exactly the
# exclusion that keeps Family 29 safe. Two families, two contracts.
#
# NOT a subject: the bare `Edit` and `Write` tool entries under **File
# operations**. They name tools rather than paths — `Write` there is a real
# grant, and reporting it would push a maintainer to revoke it. The
# extraction requires a parenthesised argument for exactly this reason.
#
# 32b — THE RETIRED LIST MUST STAY DISJOINT FROM THE CANONICAL SET.
# `/ductus:configure` step 4 lists entries the framework once shipped and
# now removes (passed to `merge-permissions` as `revoke`). An entry that
# appears in both that list and the canonical allow set in step 2 would be
# removed and re-added on every run: the merge could never reach a fixed
# point, so `unchanged` would never fire and the mtime-preserving
# short-circuit would be dead. `merge-permissions` rejects such a call at
# runtime (`ConflictingRevoke`), which turns a maintainer's editing slip
# into a failure in every adopter's `/ductus:configure` run rather than in
# ours. This check moves that failure back to maintainer time, where the
# fix is one edit: retiring an entry means deleting it from step 2 in the
# same commit that adds it to step 4.
#
# METHOD. One host, because only one can express the shape:
#
#   32  configure/claude.md — bullet entries of the form `- `Write(...)``
#       inside BOTH the `permissions.allow` and `permissions.deny`
#       sections, whose bounds are derived from the numbered headings
#       rather than hardcoded line numbers. Anchoring to the bullet form is
#       load-bearing: §5's prose names both removed entries as a
#       counter-example, and a looser grep would report that explanation as
#       a violation of itself — the same trap Family 29 documents.
#
# SKIPPED, and why the skip is not a coverage gap:
#   - configure/opencode.md scopes by TOOL, not path (`"edit": "allow"`);
#     there is no path-scoped form to get wrong.
#   - configure/auggie.md carries no file-permission entries at all.
#   - configure/antigravity.md omits `read_file`/`write_file` entries
#     deliberately — it auto-allows in-workspace file access, so a
#     path-scoped file grant has nothing to express.
# A host whose format gains path-scoped file permissions later is a new
# subject here, not a silent pass.
#
# NOT a subject: `.claude/commands/ductus/configure.md`. It is generator
# output, so a finding there would report the generator rather than a
# defect, and its sync with the source is already gated by check-zero —
# the same reasoning Families 28 and 29 apply to their generated copies.
#
# An empty extraction on a subject that exists is a finding, not a pass: a
# canonical set that parses to zero entries means the parse broke, and
# reporting clean on it is the false green /audit exists to prevent.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family permission-entry-shape

CLAUDE="framework/bootstrap/configure/claude.md"

if [ ! -f "$CLAUDE" ]; then
  emit "$CLAUDE" "not found — this family cannot examine its subject, which is not the same as clean" \
    "restore the file or update this family's paths"
  exit 1
fi

# Heading line numbers. Matching `^[0-9]+\.` rather than a literal step
# number keeps the bounds derived, so renumbering the command's steps moves
# the window instead of breaking it — Family 29's reasoning, and its bug.
allow_start="$(grep -nE '^[0-9]+\. Canonical `permissions\.allow` entries' "$CLAUDE" | cut -d: -f1 | head -1)"
deny_start="$(grep -nE '^[0-9]+\. Canonical `permissions\.deny` entries' "$CLAUDE" | cut -d: -f1 | head -1)"

if [ -z "$allow_start" ] || [ -z "$deny_start" ]; then
  emit "$CLAUDE" "could not locate the canonical allow/deny section headings — the parse is broken, not the file" \
    "check the \"N. Canonical \`permissions.allow\` entries:\" heading format"
  exit "$drift"
fi

# The deny section ends at the next numbered heading after it. Falls back to
# end-of-file when the deny section is last, so a reordering of the command's
# steps cannot silently shrink the window to nothing.
deny_end="$(awk -v s="$deny_start" 'NR>s && /^[0-9]+\. / { print NR; exit }' "$CLAUDE")"
if [ -z "$deny_end" ]; then
  deny_end="$(($(wc -l < "$CLAUDE") + 1))"
fi

entry_count=0

# check_section START END LABEL — report every path-scoped `Write(...)`
# bullet in the half-open line range, and add its entry count to the total.
check_section() {
  section_start="$1"
  section_end="$2"
  label="$3"
  section_entries=0

  while IFS=: read -r lineno entry; do
    [ -n "${entry:-}" ] || continue
    section_entries=$((section_entries + 1))
    case "$entry" in
      # A parenthesised argument is what makes it path-scoped. The bare
      # `Write` tool entry has none and is deliberately not a subject.
      Write\(*\))
        # grep -n numbered the awk slice, which starts at section_start + 1;
        # shift back to a file line so the finding is clickable.
        emit "$CLAUDE:$((section_start + lineno))" \
          "$label entry \`$entry\` is path-scoped \`Write(...)\` — file permission checks match only \`Edit(path)\` rules, so this entry grants nothing and the host warns about it at every session start" \
          "use \`Edit(${entry#Write\(}\` instead (an Edit rule covers every file-editing tool, Write included), or drop the entry if an Edit rule for the same path is already present"
        ;;
    esac
  done <<EOF
$(awk -v s="$section_start" -v e="$section_end" 'NR>s && NR<e' "$CLAUDE" \
  | grep -n '^[[:space:]]*- `[A-Za-z_]*(' \
  | sed 's/^\([0-9][0-9]*\):[[:space:]]*- `\(.*\)`.*/\1:\2/')
EOF

  entry_count=$((entry_count + section_entries))
}

check_section "$allow_start" "$deny_start" "allow"
check_section "$deny_start" "$deny_end" "deny"

# 32b — the retired list and the canonical allow set must not intersect.
# Absent retired section: nothing has been retired yet, which is a valid
# state and not a parse failure, so it is skipped rather than reported.
retired_start="$(grep -nE '^[0-9]+\. Retired `permissions\.allow` entries' "$CLAUDE" | cut -d: -f1 | head -1)"
if [ -n "$retired_start" ]; then
  retired_end="$(awk -v s="$retired_start" 'NR>s && /^[0-9]+\. / { print NR; exit }' "$CLAUDE")"
  if [ -z "$retired_end" ]; then
    retired_end="$(($(wc -l < "$CLAUDE") + 1))"
  fi

  # Bullet entries in a line range, one per line, backticks stripped.
  entries_in() {
    awk -v s="$1" -v e="$2" 'NR>s && NR<e' "$CLAUDE" \
      | sed -n 's/^[[:space:]]*- `\(.*\)`.*/\1/p'
  }

  canonical_allow="$(entries_in "$allow_start" "$deny_start")"
  retired="$(entries_in "$retired_start" "$retired_end")"
  retired_count="$(printf '%s\n' "$retired" | grep -c '[^[:space:]]')"

  if [ "$retired_count" -eq 0 ]; then
    emit "$CLAUDE:$retired_start" "the retired-entries section holds no bullet entries — the parse is broken, not the list" \
      "check the \"- \`Entry\`\" bullet format inside the retired section"
  else
    while IFS= read -r retired_entry; do
      [ -n "${retired_entry:-}" ] || continue
      if printf '%s\n' "$canonical_allow" | grep -qxF "$retired_entry"; then
        emit "$CLAUDE:$retired_start" \
          "\`$retired_entry\` is listed as retired AND is still a canonical allow entry — merge-permissions would remove it and re-add it on every run, so it refuses the call outright (ConflictingRevoke) and \`/ductus:configure\` fails for every adopter" \
          "delete it from the canonical \`permissions.allow\` set in the same edit that retires it; an entry belongs to exactly one of the two lists"
      fi
    done <<EOF
$retired
EOF
  fi
fi

if [ "$entry_count" -eq 0 ]; then
  emit "$CLAUDE" "no parenthesised permission entries extracted from the canonical allow/deny sections — the parse is broken, not the set" \
    "check the \"- \`Tool(argument)\`\" bullet format inside both sections"
fi

echo "permission-entry-shape: checked $entry_count claude allow+deny entries for shape, and ${retired_count:-0} retired entries for overlap with the canonical set (opencode/auggie/antigravity skipped — none has a path-scoped file-permission form)" >&2

exit "$drift"
