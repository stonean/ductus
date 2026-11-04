#!/usr/bin/env bash
# scripts/audit/review-freshness.sh — Family 19 of /audit.
#
# No `done` spec ships with a review that predates its own code.
#
# `/{project}:review` records `reviewed-against` in `review.md`'s frontmatter
# (in the spec's `review:` block before spec 057 relocated the record).
# Nothing compared it to reality: `check-review-gate` asserts
# `last-run` is set and `blocking` is false, and `/{project}:analyze`'s
# `review-state-drift` family asserts the same two things. Both pass for a
# review recorded against a commit whose code has since changed, so a
# passing-but-stale review is indistinguishable from a current one — and
# hand-editing a `review.md` to mark findings resolved produces exactly that
# shape.
#
# That is not hypothetical. `gvrn-v0.26.2` was tagged at `334907f` while
# spec 022's review read `reviewed-against: 1f7ee722`, three commits back;
# the adopter-scope suppression shipped unreviewed and every automated check
# passed (spec 022 review, 2026-08-03).
#
# This family is the release-time half of the test `check-review-gate`
# applies at completion time, and since 2026-11-04 it is the same
# implementation: it asks the runtime's `check-review-freshness`, which makes
# the gate's own comparison (`analyze_subjects::review_freshness`), of every
# `done` spec (066 plan, *Family 19 calls the runtime*). Its earlier Python
# copy of the rule disagreed with the runtime once, for 19 of 46 `done` specs
# (066 scenario `review-staleness-on-done-specs`).
#
# Method: for each spec the `dashboard` primitive reports at `status: done`,
# read `check-review-freshness`'s answer:
#   19a `stale` — a finding naming the durable contracts (`scenarios/*.md`
#       and `data-model.md`) that no longer match `reviewed-digest`. A uniform
#       rename sweep is exempt, as at the gate.
#   19b `undeterminable` — a finding: the record carries no
#       `reviewed-digest`, or `review.md`'s frontmatter does not parse, so
#       nothing says what the review read.
#   19c no `review.md` — grandfathered, not a finding: the absent file is the
#       never-reviewed state (§text-first-artifacts), the population
#       `/{project}:analyze` and the shipped CI template grandfather too.
#   19d a `review.md` whose `last-run` is null — counted as never run, not a
#       finding: the spec never completed a review, which is the completion
#       gate's `not-reviewed`, not staleness.
#   A call that fails is a finding, never a pass.
#
# Scoped to the durable contracts, and that scoping was earned rather than
# guessed. Two wider rules were measured against this repo first:
#   * the plan's **Affected Files** — flagged 42 of 48, because old specs
#     list shared surfaces (`AGENTS.md`, `README.md`,
#     `framework/bootstrap/ductus.md`) that every later spec also touches, so
#     spec 004 read "stale" because spec 042 edited `AGENTS.md`;
#   * the spec's whole directory — flagged 31 of 48, because `tasks.md`
#     churns on every ticked checkbox and is ephemeral by construction
#     (§tasks-phase), and `plan.md` churns as Affected Files are revised.
# Both would have been disabled within a week. The durable-contract rule
# flagged 10 of 48 when it landed, caught both real failures (`gvrn-v0.26.1`
# and `gvrn-v0.26.2`, each of which added 022 scenarios after its review), and
# reported nothing on a spec whose only movement was bookkeeping. That count
# is a snapshot, not a constant: it drains as reviews are refreshed.
#
# Requires `python3`, to read the primitives' JSON, and the runtime.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family review-freshness

if ! command -v python3 >/dev/null 2>&1; then
  emit "(precondition)" "python3 not on PATH — cannot read the runtime's answers" \
    "install python3 and re-run"
  exit 1
fi

ductus_bin="$(ductus_bin)"
# An unreachable runtime is a finding, never a silent pass.
if [ -z "$ductus_bin" ]; then
  emit "(precondition)" \
    "ductus runtime not reachable — review freshness could not be compared" \
    "run /ductus to acquire the runtime, or build it with cargo build --release in runtime/"
  exit "$drift"
fi

SPECS_ROOT="specs"
if [ -f "$ROOT/.ductus/config.toml" ] || [ -f "$ROOT/.govern.toml" ]; then
  cfg="$ROOT/.ductus/config.toml"; [ -f "$cfg" ] || cfg="$ROOT/.govern.toml"
  derived="$(python3 - "$cfg" <<'PY' 2>/dev/null || true
import sys, tomllib
try:
    with open(sys.argv[1], "rb") as fh:
        data = tomllib.load(fh)
except Exception:
    sys.exit(0)
value = (data.get("paths") or {}).get("specs-root")
if isinstance(value, str) and value.strip():
    print(value.strip())
PY
)"
  [ -n "$derived" ] && SPECS_ROOT="$derived"
fi

python3 - "$ROOT" "$SPECS_ROOT" "$ductus_bin" <<'PY'
import json
import subprocess
import sys

root, specs_root, binary = sys.argv[1], sys.argv[2], sys.argv[3]
findings = []

# Coverage counters, reported unconditionally at the end: a family that
# prints nothing on a clean run is indistinguishable from one that aborted
# before it examined anything (`QUAL-CLAIM-001`).
examined = 0
grandfathered = 0
undeterminable = 0
never_run = 0
failed = 0


def emit(location, message, fix):
    # Buffered rather than printed: the coverage line is only known at the
    # end, and it renders above the findings it quantifies.
    findings.append(f"review-freshness | {location} | {message} | {fix}")


def call(*args):
    # UTF-8 explicitly, not the locale default: on Windows that is cp1252,
    # which raises on the first em-dash in a path or reason.
    return subprocess.run(
        [binary, *args],
        cwd=root,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
    )


def first_line(text):
    return (text.strip().splitlines() or ["no output"])[0]


dashboard = call("dashboard")
try:
    specs = json.loads(dashboard.stdout)["specs"]
except (ValueError, KeyError, TypeError):
    emit(
        "(precondition)",
        f"dashboard did not answer (exit {dashboard.returncode}): "
        f"{first_line(dashboard.stderr)} — the done specs could not be listed",
        f"run {binary} dashboard and read its error",
    )
    specs = None

for spec in specs or []:
    if spec.get("status") != "done":
        continue
    feature = spec["slug"]
    rel_dir = f"{specs_root}/{feature}"
    run = call("check-review-freshness", "--feature", feature)
    try:
        result = json.loads(run.stdout)
        freshness = result["freshness"]
        state = freshness["state"]
    except (ValueError, KeyError, TypeError):
        failed += 1
        emit(
            rel_dir,
            f"check-review-freshness did not answer (exit {run.returncode}): "
            f"{first_line(run.stderr)} — this spec's review was not compared",
            f"run {binary} check-review-freshness --feature {feature} and read its error",
        )
        continue
    if state == "never-run":
        if result.get("record-present"):
            never_run += 1
        else:
            grandfathered += 1
        continue
    if state == "undeterminable":
        undeterminable += 1
        emit(
            rel_dir,
            f"done spec's review freshness cannot be determined — {freshness.get('reason', '')}",
            f"re-run review against {feature}, so review.md records a reviewed-digest",
        )
        continue
    examined += 1
    if state == "stale":
        stale = freshness.get("paths", [])
        shown = ", ".join(stale[:3])
        more = f" (+{len(stale) - 3} more)" if len(stale) > 3 else ""
        emit(
            rel_dir,
            f"done spec's review is stale — {len(stale)} durable contract(s) no "
            f"longer match reviewed-digest: {shown}{more}",
            f"re-run review against {feature} before releasing",
        )

# The coverage claim, on every run — clean, findings, or an empty corpus — to
# stderr like every other family's, so run_check passes it through above the
# findings it quantifies.
print(
    f"review-freshness: examined {examined} spec(s) at status: done by reviewed-digest; "
    f"{grandfathered} grandfathered (no review.md); {undeterminable} undeterminable; "
    f"{never_run} never run; {failed} not answered",
    file=sys.stderr,
)
for finding in findings:
    print(finding)

sys.exit(1 if findings else 0)
PY
exit $?
