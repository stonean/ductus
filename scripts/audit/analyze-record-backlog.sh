#!/usr/bin/env bash
# scripts/audit/analyze-record-backlog.sh — Family 37 of /audit.
#
# Every `done` spec that carries a review record and no analyze record. The CI
# template's analyze-record check exempts every `done` spec with no analyze
# record, so this is the part of that exempt set that drains through
# `/{project}:analyze`. A spec with neither record is counted separately (see
# THE SUBJECT below).
#
# Since spec 057 each record lives in the artifact that owns it: the review
# record in `review.md`, the analyze record in `analysis.md`, and `spec.md`
# carries neither. **The absent artifact is the never-run state**, so that is
# what this family counts. An artifact that exists but whose frontmatter does
# not parse is undeterminable, which is a finding rather than an exemption:
# collapsing it into never-run would grow the exempt set with a defect.
#
# WHY THE EXEMPTION EXISTS, AND WHY IT NEEDS THIS. Spec 047 gave
# `/{project}:analyze` a durable record so the pipeline's second gate could be
# enforced; before it, a spec that had passed both gates and one that had
# passed only the review were byte-identical on disk. Every `done` spec
# written before that record existed therefore has no analyze record, and the
# CI template's record check exempts them.
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
# THE SUBJECT IS `done` SPECS WITH A `review.md`. A `done` spec with *neither*
# artifact predates `/{project}:review` too and is already grandfathered by
# that family; it is counted separately and reported as such rather than
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

specs_root="$(specs_root)"

# The root is committed text. One resolving outside the repository is
# reported and no record under it is read (026 plan, BE-INPUT-004).
contained "$ROOT" "$specs_root"
case $? in
  1)
    emit "$specs_root/" \
      "the configured spec root resolves outside the repository, so no audit record under it was read" \
      "set [paths] specs-root in .ductus/config.toml to a directory inside the repository"
    exit "$drift"
    ;;
  2)
    containment_unchecked "$specs_root/"
    exit "$drift"
    ;;
esac

records="$(printf '%s\n' "$corpus" | SPECS_ROOT="$specs_root" python3 -c '
import os, sys

root = os.environ["SPECS_ROOT"]
repo = os.path.realpath(".")

def record_state(path):
    """Whether an audit artifact carries a record: absent / present / unreadable.

    Three states, never two. The artifact being **absent** is the never-run
    signal the constitution names, so it is the grandfathered population this
    family counts. An artifact that exists and whose frontmatter does not
    parse is **undeterminable** — a real gap in what this run could see, and
    folding it into never-run would grow the exempt set with a defect.
    """
    try:
        with open(path, encoding="utf-8") as handle:
            lines = handle.read().splitlines()
    except FileNotFoundError:
        return "absent"
    except OSError:
        return "unreadable"
    if not lines or lines[0].strip() != "---":
        return "unreadable"
    for line in lines[1:]:
        if line.strip() == "---":
            return "present"
    return "unreadable"

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
    # A record resolving outside the repository through a symbolic link is
    # reported and never opened (026 plan, BE-INPUT-004).
    outward = [name for name in ("review.md", "analysis.md")
               if os.path.commonpath([repo, os.path.realpath(os.path.join(root, slug, name))]) != repo]
    if outward:
        print("outside\t%s\t" % os.path.join(root, slug, outward[0]))
        continue
    review = record_state(os.path.join(root, slug, "review.md"))
    analysis = record_state(os.path.join(root, slug, "analysis.md"))
    if "unreadable" in (review, analysis):
        which = "review.md" if review == "unreadable" else "analysis.md"
        print("unreadable\t%s\t" % os.path.join(root, slug, which))
        continue
    examined += 1
    if analysis == "present":
        continue
    if review == "absent":
        # Predates the review record too; already grandfathered there, and it
        # drains through a different command. Counted, never merged.
        never_reviewed += 1
        continue
    print("backlog\t%s\t%s" % (os.path.join(root, slug, "spec.md"), slug))
print("counts\t%d\t%d" % (examined, never_reviewed))
')"

examined=0
never_reviewed=0
backlog=0
backlog_specs=""
saw_counts=0
while IFS=$'\t' read -r kind f1 f2; do
  case "$kind" in
    backlog)
      backlog=$((backlog + 1))
      backlog_specs="$backlog_specs$f2"$'\n'
      ;;
    outside)
      emit "$f1" \
        "audit artifact resolves outside the repository, so it was not read and this spec was not examined" \
        "replace the symbolic link with the record, or remove it"
      ;;
    unreadable)
      emit "$f1" \
        "audit artifact exists but carries no readable frontmatter record — this spec was not examined" \
        "repair the artifact or re-run the command that writes it; undeterminable is not the never-run state and must not join the exempt set"
      ;;
    counts)
      examined="$f1"
      never_reviewed="$f2"
      saw_counts=1
      ;;
  esac
done <<< "$records"

# The scan emits its counts record last and unconditionally, so its absence
# means the scan did not finish — a python failure, a broken pipe. Without
# this the failure is silent and generous: `records` comes back empty, the
# loop matches nothing, `backlog` stays 0, and 0 is at or below every
# baseline, so the ratchet reports clean having examined nothing. That is
# `QUAL-CLAIM-001` in the family whose whole job is to keep a grandfathered
# set honest, and it is the third instance of this exact shape found by
# reviewing this session's own new code against the rule set it enforces.
#
# An `examined` of zero is NOT the guard: a corpus with no `done` specs is
# legitimately empty, and its backlog of zero is a true answer.
if [ "$saw_counts" -eq 0 ]; then
  emit "$SELF" \
    "the frontmatter scan produced no counts record — it did not finish, so the backlog of ${backlog} describes nothing" \
    "run the script directly and inspect the python stage; a scan that did not complete must not be read as an empty backlog"
fi

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
echo "analyze-record: ${examined} done spec(s) examined; ${backlog} carry a review.md and no analysis.md (the grandfathered backlog, baseline ${baseline:-unreadable}); ${never_reviewed} have neither and drain through the review command instead" >&2

exit "$drift"
