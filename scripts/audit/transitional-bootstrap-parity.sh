#!/usr/bin/env bash
# scripts/audit/transitional-bootstrap-parity.sh — Family 21 of /audit.
#
# The retired bootstrap path still serves the current bootstrap.
#
# Every adopter's installed `/govern` command carries a hardcoded self-update
# fetch at the path it was published under:
#
#   https://raw.githubusercontent.com/stonean/govern/main/framework/bootstrap/govern.md
#
# GitHub resolves the *repository* half of that URL after a rename — verified
# against a known-renamed repo, raw.githubusercontent.com and
# codeload.github.com both answer 200 on the old owner/name with no redirect
# hop. The *path* half is not covered by anything: a renamed file simply does
# not exist at its old path, and raw answers 404.
#
# That 404 is not a soft failure. The bootstrap those adopters are running
# aborts the run on a failed self-update fetch and does not continue — so it
# never reaches §Pre-run Migrations, and the migration that would converge them
# onto the new name can never execute. Deleting the old path strands every
# existing adopter permanently, with no signal on this side.
#
# So `framework/bootstrap/govern.md` stays, byte-identical to
# `framework/bootstrap/ductus.md`. An old adopter fetches it, finds it differs
# from their installed copy, writes it, and restarts into the new instructions;
# their next run migrates them. Identical content is what makes that work: the
# self-update check is a byte-compare, so a shim that merely *described* the
# move would be written to their command file and leave them with a stub.
#
# Two copies drift. This family is the check that they do not.
#
# Removal condition: this file and the copy it guards can be deleted once no
# adopter can still be running a pre-rename bootstrap — practically, one full
# deprecation window after the first ductus-v tag. Deleting them earlier is the
# same permanent strand as never having added them.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family transitional-bootstrap-parity

CURRENT="$ROOT/framework/bootstrap/ductus.md"
RETIRED="$ROOT/framework/bootstrap/govern.md"

if [ ! -f "$CURRENT" ]; then
  emit "framework/bootstrap/ductus.md" "current bootstrap is missing" \
    "restore it — every adopter's self-update fetch resolves to this file"
  exit "$drift"
fi

if [ ! -f "$RETIRED" ]; then
  emit "framework/bootstrap/govern.md" \
    "the retired bootstrap path is absent — every pre-rename adopter's self-update fetch 404s and their run aborts before migrations" \
    "restore it as a byte-identical copy of framework/bootstrap/ductus.md (see this script's header for why it cannot simply be dropped)"
  exit "$drift"
fi

if ! cmp -s "$CURRENT" "$RETIRED"; then
  emit "framework/bootstrap/govern.md" \
    "the retired bootstrap path has drifted from framework/bootstrap/ductus.md" \
    "copy framework/bootstrap/ductus.md over it — an adopter's self-update byte-compare writes this file to their command, so stale content ships to them verbatim"
fi

exit "$drift"
