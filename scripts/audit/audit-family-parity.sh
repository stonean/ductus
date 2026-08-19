#!/usr/bin/env bash
# scripts/audit/audit-family-parity.sh — Family 28 of /audit.
#
# /audit runs the families run-all.sh registers. framework/commands/audit.md
# enumerates the families for the maintainer who reads the command.
# scripts/audit/README.md §Scripts lists the scripts. Nothing held the three
# together, and they drifted: run-all.sh registered 25 families (1-2, 4-26)
# while audit.md's numbered list stopped at 23, so Families 24, 25, and 26 were
# absent from it in any form.
#
# The three missing were the three added most recently, which is the direction
# this drift always runs — a family is written, wired into run-all.sh because
# that is what makes it *execute*, and the doc update is the step with no
# consequence if skipped.
#
# The cost is not cosmetic. audit.md is the maintainer's map of what the release
# gate asserts. A map that under-reports invites the conclusion that a concern
# is unchecked when it is checked; in the other direction — a script dropped
# from run-all.sh while its entry survives in audit.md — a family that no longer
# runs reads as still running, which is worse. Both are failures of the same
# missing binding, so both are reported, separately, because the repairs differ.
#
# This is the shape Family 2 (manifest parity), Family 18 (marker-list parity),
# and Family 23 (sweep-target manifest parity) already exist for. /audit simply
# never had one covering *itself*.
#
# METHOD. Both sets are derived, never hardcoded — a hardcoded expectation would
# be a third copy of exactly the fact under test:
#
#   28a Registered pairs `<family-number> <script-path>` from run-all.sh's
#       `run_check "Family N — ..." "scripts/audit/x.sh"` lines.
#   28b Documented pairs from audit.md's numbered entries, anchored to the full
#       `N. Run `scripts/audit/x.sh` (Family N` form. Anchoring to the whole
#       line rather than to the digits is what keeps prose mentions of a family
#       number out of the set — audit.md's boundary section and several entries
#       reference other families by number in running text.
#
#       Two entry spellings are in use and both count: families 1-13 are bare
#       (`(Family 7).`), families 14+ carry a description (`(Family 14 — ...)`).
#       Matching only the second was this family's own first-draft bug: it
#       reported the twelve older entries as undocumented, which would have sent
#       a maintainer rewriting a list that was already correct. A parity check
#       whose findings are wrong is worse than no parity check.
#
#       The list number is deliberately NOT read as the family number. They are
#       offset by one — entry `1.` is check-zero, a precondition rather than a
#       family, so entry `2.` is Family 1 — and the parenthetical is the
#       authority on both sides.
#   28c Every registered script appears in README.md §Scripts.
#
# Comparing pairs rather than bare numbers means a doc entry that names the
# right family number against the wrong script is caught too.
#
# Retired numbers need no special case. Family 3 (registry equivalence) was
# retired with the workflows feature (spec 043) and its number is permanently
# spent; it appears in neither list, so the sets agree and nothing fires. Family
# numbers are stable identifiers, so a gap is the correct state.
#
# NOT a subject: .claude/commands/ductus/audit.md. It is generator output, so
# auditing it would report the generator rather than a defect; its sync is
# already gated by check-zero and the generators workflow.
#
# This family appears in its own lists, which is correct rather than circular —
# a family that exempted itself would be the one entry the check could never
# catch.
#
# An empty derivation on either side is a finding, not a pass: two empty sets
# compare equal, which is the precise false green /audit exists to prevent.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family audit-family-parity

RUN_ALL="scripts/audit/run-all.sh"
AUDIT_MD="framework/commands/audit.md"
README="scripts/audit/README.md"

for f in "$RUN_ALL" "$AUDIT_MD" "$README"; do
  if [ ! -f "$f" ]; then
    emit "$f" "not found — this family cannot examine its subject, which is not the same as clean" \
      "restore the file or update this family's paths"
    exit 1
  fi
done

# 28a — registered pairs.
registered="$(
  sed -n 's/^run_check "Family \([0-9][0-9]*\) —[^"]*" "\([^"]*\)".*/\1 \2/p' "$RUN_ALL" \
    | sort -u
)"

# 28b — documented pairs, anchored to the full numbered-entry form.
documented="$(
  sed -n 's/^[0-9][0-9]*\. Run `\(scripts\/audit\/[a-z0-9-]*\.sh\)` (Family \([0-9][0-9]*\).*/\2 \1/p' "$AUDIT_MD" \
    | sort -u
)"

reg_count="$(printf '%s\n' "$registered" | grep -c . || true)"
doc_count="$(printf '%s\n' "$documented" | grep -c . || true)"

if [ "$reg_count" -eq 0 ]; then
  emit "$RUN_ALL" "no registered families extracted — the parse is broken, not the registry" \
    "check the run_check \"Family N — ...\" line format"
fi
if [ "$doc_count" -eq 0 ]; then
  emit "$AUDIT_MD" "no documented families extracted — the parse is broken, not the list" \
    "check the \"N. Run \`scripts/audit/x.sh\` (Family N\" entry format"
fi
# Comparing after an empty extraction would report agreement between two empty
# sets, so stop at the extraction failure instead.
if [ "$drift" -ne 0 ]; then
  exit "$drift"
fi

only_registered="$(comm -23 <(printf '%s\n' "$registered") <(printf '%s\n' "$documented"))"
only_documented="$(comm -13 <(printf '%s\n' "$registered") <(printf '%s\n' "$documented"))"

while read -r num script; do
  [ -n "${num:-}" ] || continue
  emit "$AUDIT_MD" "Family $num ($script) runs but is not in the command's family list" \
    "add a \"$num. Run \`$script\` (Family $num — ...)\" entry to $AUDIT_MD"
done <<EOF
$only_registered
EOF

while read -r num script; do
  [ -n "${num:-}" ] || continue
  emit "$AUDIT_MD" "Family $num ($script) is documented but no longer registered — it reads as running when it does not" \
    "remove the entry from $AUDIT_MD, or re-register the script in $RUN_ALL"
done <<EOF
$only_documented
EOF

# 28c — every registered script is listed in README.md §Scripts.
while read -r num script; do
  [ -n "${script:-}" ] || continue
  base="$(basename "$script")"
  if ! grep -qF -- "\`$base\`" "$README"; then
    emit "$README" "Family $num's script $base is registered but not listed in §Scripts" \
      "add a \"- \`$base\` — Family $num.\" line to $README"
  fi
done <<EOF
$registered
EOF

echo "audit-family-parity: compared $reg_count registered against $doc_count documented family entries" >&2

exit "$drift"
