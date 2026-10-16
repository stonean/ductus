#!/usr/bin/env bash
# scripts/audit/promotion-coverage.sh — Family 38 of /audit.
#
# How much of AGENTS.md has been routed to the constitution, reported as a
# coverage line rather than as a gate.
#
# §drift-prevention's *Shared knowledge stays in git* routes a learning by
# population, and the first destination is the constitution. Nothing reported
# how much of AGENTS.md had actually gone there, and *not having promoted* is
# invisible by construction: a universal entry still sitting in AGENTS.md
# looks exactly like one correctly judged project-only, because neither
# carries a marker and a per-entry marker is the authored input
# §design-principles rejects.
#
# THE CHECK IS A PRIMITIVE; THIS IS THE ENTRY POINT. The counting lives in
# `check-promotion-coverage`, which reuses the tested comment- and
# fence-aware bullet grammar the inbox primitives already share. Re-deriving
# it here in awk or python would be a second implementation of markdown
# structure parsing — the thing §runtime-boundary principle 3 names.
#
# A NOTICE, NEVER A GATE, and the split is deliberate:
#
#   * the coverage line goes to stderr and never touches the exit code, which
#     is Family 19's existing shape. Promotion has to stay free for the reason
#     §brownfield-inbox gives for capture — gating the release audit on how
#     much is unrouted would make the honest choice between a growing backlog
#     and a silent one push toward silence.
#   * an unmatched table key IS a finding. It means the classification table
#     names an entry that no longer reads that way, so a recorded verdict
#     points at nothing: real drift between two artifacts, which is what
#     /audit is for. A lead-phrase key degrades loudly to not matching at all,
#     where a positional identifier would have degraded silently to matching
#     whichever entry later occupied its slot.
#   * a missing section IS a finding, because a denominator that quietly
#     shrinks when a section is renamed moves coverage in the flattering
#     direction.
#
# MEASURED 2026-09-15: 121 rule-bearing across the four sections, 98 table
# rows all matched, 74 pointer-citing, 119 classified, 2 unclassified,
# 0 unmatched keys, 0 missing sections.
#
# Bash 3.2 compatible (macOS system bash). The runtime emits JSON; python3
# renders tab-separated records and the shell renders those through `emit`,
# per the convention ./README.md records.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family promotion-coverage

# The subject is this repository's own rules file and the classification table
# kept by the spec that owns constitution content. Both are maintainer-owned,
# which is why this family lives in /audit rather than on an adopter surface.
RULES_FILE="AGENTS.md"
TABLE_FILE="specs/050-constitution/plan.md"
TABLE_SECTION="Classification"
POINTER_LINK="framework/constitution.md#"

ductus_bin=""
if [ -x .ductus/bin/ductus ]; then
  ductus_bin=".ductus/bin/ductus"
elif [ -x runtime/target/release/ductus ]; then
  ductus_bin="runtime/target/release/ductus"
elif command -v ductus > /dev/null 2>&1; then
  ductus_bin="ductus"
fi

# An unreachable runtime is a finding, never a silent pass: a family that
# cannot run must not exit 0 wearing the costume of one that passed.
if [ -z "$ductus_bin" ]; then
  emit "(precondition)" \
    "ductus runtime not reachable — the promotion-coverage line could not be computed" \
    "run /ductus to acquire the runtime, or build it with cargo build --release in runtime/"
  exit "$drift"
fi

if ! payload="$("$ductus_bin" check-promotion-coverage \
  --rules-file "$RULES_FILE" \
  --section Workflow \
  --section Gotchas \
  --section Boundaries \
  --section "Design Principles" \
  --table-file "$TABLE_FILE" \
  --table-section "$TABLE_SECTION" \
  --pointer-link "$POINTER_LINK" 2>/dev/null)"; then
  emit "(precondition)" \
    "ductus check-promotion-coverage failed to execute" \
    "run $ductus_bin check-promotion-coverage directly to see the error"
  exit "$drift"
fi

records="$(
  printf '%s' "$payload" | RULES_FILE="$RULES_FILE" TABLE_FILE="$TABLE_FILE" python3 -c '
import json, os, sys

data = json.load(sys.stdin)
rules_file = os.environ["RULES_FILE"]
table_file = os.environ["TABLE_FILE"]

for key in data.get("unmatched-keys", []):
    print("\t".join([
        table_file,
        "classification key matches no entry: %s" % key,
        "the entry was reworded — re-key the row to its current lead phrase in %s" % rules_file,
    ]))

for section in data.get("missing-sections", []):
    print("\t".join([
        rules_file,
        "named section not found: %s" % section,
        "the section was renamed or removed — update the family scope so the denominator does not shrink silently",
    ]))

# The coverage line. Above any findings, exit-code-neutral, and rendered on
# every run including a fully-routed one: examined-and-fully-routed and
# not-computed must not read alike.
if data.get("table-state") == "no-table":
    sys.stderr.write(
        "promotion-coverage: no classification table at %s — coverage not computed\n" % table_file)
else:
    sys.stderr.write(
        "promotion-coverage: %d rule-bearing across 4 section(s) — %d table-keyed, "
        "%d pointer-citing, %d classified => %d unclassified\n" % (
            data.get("rule-bearing", 0), data.get("table-keyed", 0),
            data.get("pointer-citing", 0), data.get("classified", 0),
            data.get("unclassified", 0)))
    for lead in data.get("unclassified-entries", []):
        sys.stderr.write("promotion-coverage:   unclassified — %s\n" % lead)
'
)" || {
  emit "(precondition)" \
    "could not parse check-promotion-coverage output" \
    "run $ductus_bin check-promotion-coverage and inspect the JSON"
  exit "$drift"
}

while IFS="$(printf '\t')" read -r location message fix; do
  [ -n "$location" ] || continue
  emit "$location" "$message" "$fix"
done << EOF
$records
EOF

exit "$drift"
