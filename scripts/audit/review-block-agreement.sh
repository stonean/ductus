#!/usr/bin/env bash
# scripts/audit/review-block-agreement.sh — Family 31 of /audit.
#
# A spec's frontmatter `review:` block must agree with its own review.md.
#
# The same review is recorded twice: `spec.md`'s block carries last-run,
# reviewed-against, the three counts, blocking, and any waivers; the review.md
# beside it carries reviewed-at, reviewed-against, and the same three counts.
# Nothing held the two together, and they drifted — 031 and 041 both carried
# should-violations: 1 in spec.md while their reports recorded 0, for weeks,
# invisible because every gate reads exactly one of the two files.
# check-review-gate and /ductus:analyze's review-drift check read the block;
# Family 19 resolves reviewed-against but never compares the counts on either
# side of it. A stale non-zero count reads as outstanding work that does not
# exist; a stale zero would hide real findings from every gate that trusts it.
#
# THIS SCRIPT IS AN ENTRY POINT, NOT THE CHECK. The check is the
# `check-review-agreement` runtime primitive, which deserializes both
# frontmatter blocks with the runtime's own YAML reader. That is deliberate and
# is the shape /audit families should take: the constitution's §runtime-boundary
# makes runtime eligibility a default rather than a permission, and a script
# that parses frontmatter has already failed principle 3 regardless of whether
# it reaches for awk or an embedded interpreter.
#
# This family is the worked example of the cost. Its first implementation was a
# python3 heredoc whose hand-rolled `scalar()` used `\s*` after the key name;
# because `\s` matches a newline, an empty value walked the match onto the next
# line and returned *that* line's content, so it reported 031 — whose waiver is
# recorded correctly — as carrying an orphan waiver. The primitive's test
# `an_empty_value_never_reads_the_next_line` is that bug, pinned.
#
# Requires `python3` only to render the primitive's JSON as finding rows.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family review-block-agreement

ductus_bin=""
if [ -x .ductus/bin/ductus ]; then
  ductus_bin=".ductus/bin/ductus"
elif [ -x runtime/target/release/ductus ]; then
  ductus_bin="runtime/target/release/ductus"
elif command -v ductus > /dev/null 2>&1; then
  ductus_bin="ductus"
fi

# An unreachable runtime is a finding, never a silent pass. A family that
# cannot run must not exit 0 wearing the costume of one that passed.
if [ -z "$ductus_bin" ]; then
  emit "(precondition)" \
    "ductus runtime not reachable — the review-block agreement check could not run" \
    "run /ductus to acquire the runtime, or build it with cargo build --release in runtime/"
  exit "$drift"
fi

if ! payload="$("$ductus_bin" check-review-agreement 2>/dev/null)"; then
  emit "(precondition)" \
    "ductus check-review-agreement failed to execute" \
    "run $ductus_bin check-review-agreement directly to see the error"
  exit "$drift"
fi

records="$(
  printf '%s' "$payload" | python3 -c '
import json, sys

data = json.load(sys.stdin)

# guidance is set when the run enumerated specs but nothing carried both
# records: comparing nothing reports agreement, so this is an enumeration
# failure rather than a clean corpus.
guidance = data.get("guidance", "")
if guidance:
    print("\t".join([
        data.get("specs-root", "specs") + "/",
        guidance,
        "check that the spec root resolves and that spec.md / review.md carry frontmatter",
    ]))

for finding in data.get("findings", []):
    print("\t".join([
        finding.get("location", ""),
        finding.get("message", ""),
        finding.get("fix", ""),
    ]))

# A spec the primitive could not read at all. Reported so an empty findings
# list is never mistaken for a verified corpus.
for skip in data.get("skipped", []):
    print("\t".join([
        skip.get("path", ""),
        "not examined — " + skip.get("reason", ""),
        "repair the file so its review record can be compared",
    ]))

examined = len(data.get("examined", []))
single = len(data.get("single-sided", []))
print(
    f"review-block-agreement: compared {examined} spec(s) carrying both a "
    f"review block and a review.md; {single} carried only one and belong to "
    f"Family 19 / check-review-gate",
    file=sys.stderr,
)
'
)" || {
  emit "(precondition)" \
    "could not render check-review-agreement output" \
    "run $ductus_bin check-review-agreement directly to inspect the payload"
  exit "$drift"
}

while IFS=$'\t' read -r location message fix; do
  [ -n "${location:-}" ] || continue
  emit "$location" "$message" "$fix"
done <<EOF
$records
EOF

exit "$drift"
