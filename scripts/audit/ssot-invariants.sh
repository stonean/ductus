#!/usr/bin/env bash
# scripts/audit/ssot-invariants.sh — Family 6 of /audit.
#
# Detect duplicate normative rule text across framework artifacts. A rule
# that should have one canonical location is checked here; if its
# distinctive text appears in another file without a back-reference, the
# audit surfaces it as a finding so the duplicate can be replaced with a
# pointer to the canonical home.
#
# v1 curated list (script header — add new entries here):
#
#   1. Status state machine: the sequence
#      `draft → clarified → planned → in-progress → done`
#      Canonical: framework/constitution.md §spec-lifecycle
#      Family 1b already checks the pipeline diagram visits the same five
#      states in the same order across docs; this check ensures the
#      *textual rule* (full sentence describing the state machine) lives
#      in one canonical home with references elsewhere, not restated.
#
#   2. Back-edge ownership: the rule that /amend owns both back-edges
#      (clarified/planned/in-progress → draft; done → in-progress).
#      Canonical: framework/constitution.md §spec-lifecycle
#      Family 1c already checks that amend.md and target.md reference each
#      back-edge (presence, not wording); this check is the SSOT companion.
#
#   3. Open-question counting rule: how the framework counts entries in
#      a ## Open Questions section (top-level list items or
#      **Bold-prefix**-style headings; placeholder lines like
#      `*None — all resolved.*` count as zero).
#      Canonical: framework/commands/clarify.md §Gate
#
# Implementation: v1 emits no findings — the curated list is documented
# above as a planning artifact, but the textual-duplicate detection
# requires distinct-enough phrase patterns to reliably distinguish "the
# rule restated" from "the rule referenced." Building those patterns
# requires iteration against real duplicates as they surface. A follow-on
# scenario operationalizes the check once concrete forcing cases exist;
# until then, the script is a placeholder that exits 0 and reminds the
# maintainer (via this header) which rules are SSOT-tracked.
#
# The design-principles tension here is explicit: an empty check looks
# like a violation of "never depend on human discipline." Accepted for v1
# because Families 1b and 1c already cover the diagram's state order and
# the back-edge references from different angles (presence patterns vs
# structural equality). This file's role is to be the named home where the
# *textual-rule* check eventually lives — promotion happens when there's
# a concrete duplicate to write the pattern against.
#
# So the family reports that skip on every run: one stderr line saying that
# detection is not implemented (026 AC8) and that it examined zero rules.
# The exit status stays 0 (owner decision) — a skip is not a finding — but a
# clean exit here must never be read as "no duplicated rule text exists", and
# the stderr line is what keeps the two apart.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family ssot-invariants

# v1: no findings emitted. Header documents the curated list. Exit 0, with
# the skip stated rather than silent.
echo "ssot-invariants: skipped — detection not implemented (026 AC8); 0 rules examined" >&2
exit 0
