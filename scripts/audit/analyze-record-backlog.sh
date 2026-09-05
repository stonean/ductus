#!/usr/bin/env bash
# scripts/audit/analyze-record-backlog.sh — Family 37 of /audit.
#
# Every `done` spec that carries a `review:` block and no `analyze:` one — the
# exact population the `analyze-state-drift` check family grandfathers.
#
# WHY THE EXEMPTION EXISTS, AND WHY IT NEEDS THIS. Spec 047 gave
# `/{project}:analyze` a durable record so the pipeline's second gate could be
# enforced; before it, a spec that had passed both gates and one that had
# passed only the review were byte-identical on disk. Every `done` spec
# written before that record existed therefore has no `analyze:` block, and
# the drift family exempts them.
#
# 046 refused exactly this shape of exemption for scenario questions — "a
# sanctioned hiding place is worse than the gap it papers over" — and the
# criterion-label check backfilled the corpus rather than grandfathering it.
# So the precedent runs against exempting, and the difference is what a
# backfill would have to assert. A criterion label is **derivable from the
# artifact**: that backfill computed a value that was already true. An analyze
# record asserts *that a run happened*, which nothing on disk can substantiate.
# Backfilling it would mean writing an unverified claim into the field a later
# gate trusts — the precise failure the record was added to prevent, committed
# by the mechanism itself. So the exemption stands, and this family is the
# price of it: the set is **named, counted, and shrinking** instead of silent.
#
# It cannot grow. `check-review-gate` has no grandfather clause, so no spec can
# reach `done` from here without a record; every member of this set predates
# the field, and re-analyzing one removes it permanently. A count that goes up
# means something is wrong with that claim, which is itself worth knowing.
#
# A RATCHET, NOT A WALL — and this suite has no advisory tier, which is what
# forces the design. `emit` sets `drift` and every family exits on it, so a
# family that reported 54 pre-existing specs as findings would red-line every
# run forever and be learned-ignored, which is worse than not checking. But a
# family that only printed a number could never fail, and a check that cannot
# fail is not a check.
#
# So the backlog is held against a committed high-water mark
# (`analyze-record-baseline.txt`). At or below it: clean, with the count on
# stderr. Above it: a finding, because the backlog **cannot legitimately
# grow** — `check-review-gate` has no grandfather clause, so nothing can reach
# `done` without a record, and every member of the set predates the field.
# Growth means the gate was bypassed, which is exactly the defect the record
# was added to prevent, recurring.
#
# The maintainer lowers the baseline as specs are re-analyzed; the file is the
# ratchet's pawl. Lowering it is the only way to make the set smaller on
# paper, and re-analyzing is the only way to make it smaller in fact, so the
# two cannot drift apart without this family saying so.
#
# These specs are not defective. They were completed correctly under the rules
# that existed when they were completed, and re-litigating that is not what
# this family is for.
#
# THE SUBJECT IS `done` SPECS WITH A `review:` BLOCK. A `done` spec with
# *neither* block predates `/{project}:review` too and is already grandfathered
# by that family; it is counted separately and reported as such rather than
# folded in, because the two populations drain through different commands and
# a single number would hide which.
#
# An empty spec corpus is a finding, never a pass: a family that enumerated
# nothing must not exit like one that examined everything.
#
# Bash 3.2 compatible (macOS system bash). The frontmatter scan computes in
# python3 and renders tab-separated records the shell feeds to `emit` — the
# convention ./README.md records for the families that compute in python.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family analyze-record

SELF="scripts/audit/analyze-record-backlog.sh"

bin="$(ductus_bin)"
if [ -z "$bin" ]; then
  emit "$SELF" \
    "no ductus runtime reachable — the spec corpus could not be enumerated" \
    "build the runtime (cargo build --release --manifest-path runtime/Cargo.toml) or run /ductus to acquire the pinned binary"
  exit "$drift"
fi

corpus="$(spec_corpus "$bin")"
if [ -z "$corpus" ]; then
  emit "$SELF" \
    "the spec corpus enumerated no features — nothing was examined for an analyze record" \
    "confirm the runtime's dashboard resolves and that the spec root is populated"
  exit "$drift"
fi

specs_root="$(sed -n -E 's/^[[:space:]]*specs-root[[:space:]]*=[[:space:]]*"([^"]+)".*/\1/p' .ductus/config.toml 2> /dev/null | head -1)"
[ -n "$specs_root" ] || specs_root="specs"

records="$(printf '%s\n' "$corpus" | SPECS_ROOT="$specs_root" python3 -c '
import os, sys

root = os.environ["SPECS_ROOT"]

def top_level_keys(path):
    """Unindented `key:` names in the frontmatter block, or None if unreadable."""
    try:
        with open(path, encoding="utf-8") as handle:
            lines = handle.read().splitlines()
    except OSError:
        return None
    if not lines or lines[0].strip() != "---":
        return None
    keys = []
    for line in lines[1:]:
        if line.strip() == "---":
            return keys
        if line and not line[:1].isspace() and ":" in line:
            keys.append(line.split(":", 1)[0].strip())
    return None

examined = 0
never_reviewed = 0
for row in sys.stdin:
    row = row.rstrip("\n")
    if not row:
        continue
    parts = row.split("\t")
    if len(parts) < 2:
        continue
    slug, status = parts[0], parts[1]
    if status != "done":
        continue
    path = os.path.join(root, slug, "spec.md")
    keys = top_level_keys(path)
    if keys is None:
        print("unreadable\t%s\t" % path)
        continue
    examined += 1
    if "analyze" in keys:
        continue
    if "review" not in keys:
        # Predates the review record too; already grandfathered there, and it
        # drains through a different command. Counted, never merged.
        never_reviewed += 1
        continue
    print("backlog\t%s\t%s" % (path, slug))
print("counts\t%d\t%d" % (examined, never_reviewed))
')"

examined=0
never_reviewed=0
backlog=0
backlog_specs=""
while IFS=$'\t' read -r kind f1 f2; do
  case "$kind" in
    backlog)
      backlog=$((backlog + 1))
      backlog_specs="$backlog_specs$f2"$'\n'
      ;;
    unreadable)
      emit "$f1" \
        "done spec could not be read — its analyze record was not examined" \
        "resolve the read failure; an unexaminable spec must not be counted as clean"
      ;;
    counts)
      examined="$f1"
      never_reviewed="$f2"
      ;;
  esac
done <<< "$records"

# --- the ratchet ------------------------------------------------------------

BASELINE_FILE="scripts/audit/analyze-record-baseline.txt"
baseline="$(sed -E 's/#.*//; s/[[:space:]]//g' "$BASELINE_FILE" 2> /dev/null | grep -v '^$' | head -1)"

if [ -z "$baseline" ]; then
  emit "$BASELINE_FILE" \
    "the analyze-record baseline is missing or unreadable — the backlog of ${backlog} could not be held against anything" \
    "restore the file with a single integer: the current backlog count this family must not exceed"
elif ! printf '%s' "$baseline" | grep -qE '^[0-9]+$'; then
  emit "$BASELINE_FILE" \
    "the analyze-record baseline '$baseline' is not a bare integer — the backlog of ${backlog} could not be held against it" \
    "reduce the file to one line containing the current backlog count"
elif [ "$backlog" -gt "$baseline" ]; then
  # The backlog cannot legitimately grow: the gate has no grandfather clause,
  # so nothing can reach `done` without a record. Growth means it was bypassed.
  while IFS= read -r slug; do
    [ -z "$slug" ] && continue
    emit "$specs_root/$slug/spec.md" \
      "done spec has a review record and no analyze record, and the backlog ($backlog) now exceeds its baseline ($baseline) — the set cannot legitimately grow, so the completion gate was bypassed" \
      "run the analyze command against $slug, or — if this spec genuinely predates the record — raise $BASELINE_FILE deliberately and say why in the commit"
  done <<< "$backlog_specs"
elif [ "$backlog" -lt "$baseline" ]; then
  # Not a finding: the ratchet turned the right way. Named anyway, because a
  # baseline nobody lowers stops being a bound and becomes decoration.
  echo "analyze-record: backlog ${backlog} is below its baseline ${baseline} — lower ${BASELINE_FILE} to ${backlog} to keep the ratchet tight" >&2
fi

# The counts are the guard, and here they are also the point: the whole
# justification for grandfathering is that the exempt set is visible and
# bounded, which is a claim only a number can carry.
echo "analyze-record: ${examined} done spec(s) examined; ${backlog} carry a review record and no analyze record (the grandfathered backlog, baseline ${baseline:-unreadable}); ${never_reviewed} predate the review record too and drain through that command instead" >&2

exit "$drift"
