#!/usr/bin/env bash
# scripts/audit/review-freshness.sh — Family 19 of /audit.
#
# No `done` spec ships with a review that predates its own code.
#
# `/{project}:review` records `review.reviewed-against` in the spec
# frontmatter. Nothing compared it to reality: `check-review-gate` asserts
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
# This family is the release-time half of the same test the runtime's
# `check-review-gate` now applies at completion time. Two enforcement points
# for one rule, matching how the blocking-semantics gate is already built
# from three mutually reinforcing mechanisms rather than one.
#
# Method:
#   19a For each spec at `status: done`, read `review.reviewed-against`.
#   19b Collect the spec's **durable contracts**: `scenarios/*.md` and
#       `data-model.md`.
#   19c Emit a finding when any of them changed between `reviewed-against`
#       and HEAD.
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
# Deliberately NOT a finding:
#   - A spec with no `review:` block. Grandfathered, matching the rule
#     `/{project}:analyze` and the shipped CI template already apply — such a
#     spec predates `/{project}:review`.
#   - A spec with no scenarios and no `data-model.md`. It has no durable
#     contract beyond `spec.md`, so there is nothing this check can compare.
#   - `tasks.md`, `plan.md`, `review.md`, and `spec.md`. The first is
#     ephemeral by construction; the rest are bookkeeping `write-review` and
#     the pipeline touch on their own. Counting any of them would make every
#     review stale the moment it was recorded.
#   - A `reviewed-against` that is not a commit in this repo (a shallow
#     clone, a rewritten history). Reported as a skip, not a finding — the
#     check cannot prove staleness it cannot resolve.
#
# Requires `git` and `python3`.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family review-freshness

for tool in git python3; do
  if ! command -v "$tool" >/dev/null 2>&1; then
    emit "(precondition)" "$tool not on PATH — cannot compare reviews against history" \
      "install $tool and re-run"
    exit 1
  fi
done

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

python3 - "$ROOT" "$SPECS_ROOT" <<'PY'
import pathlib
import re
import subprocess
import sys

root = pathlib.Path(sys.argv[1])
specs_root = sys.argv[2]
findings = []


def emit(location, message, fix):
    print(f"review-freshness | {location} | {message} | {fix}")
    findings.append(location)


def frontmatter(text):
    if not text.startswith("---"):
        return ""
    end = text.find("\n---", 3)
    return text[3:end] if end != -1 else ""


def scalar(fm, key, indent=""):
    m = re.search(rf"^{indent}{re.escape(key)}:\s*(.*)$", fm, re.M)
    if not m:
        return None
    v = m.group(1).strip().strip('"\'')
    return None if v in ("", "null", "~") else v


def is_durable_contract(rel_within_feature):
    """A scenario or the data model — the artifacts a review actually reads."""
    return (
        rel_within_feature.startswith("scenarios/")
        and rel_within_feature.endswith(".md")
    ) or rel_within_feature == "data-model.md"


specs_dir = root / specs_root
for spec_path in sorted(specs_dir.glob("*/spec.md")):
    feature = spec_path.parent.name
    rel_dir = f"{specs_root}/{feature}"
    text = spec_path.read_text(encoding="utf-8")
    fm = frontmatter(text)
    if scalar(fm, "status") != "done":
        continue
    if not re.search(r"^review:", fm, re.M):
        continue  # grandfathered: predates /review
    base = scalar(fm, "reviewed-against", indent="  ")
    if not base:
        continue  # `not-reviewed` is check-review-gate's finding, not this one

    probe = subprocess.run(
        ["git", "-C", str(root), "cat-file", "-e", f"{base}^{{commit}}"],
        capture_output=True,
    )
    if probe.returncode != 0:
        emit(
            rel_dir,
            f"review names reviewed-against {base[:8]}, which is not a commit in this repo "
            "— staleness cannot be determined",
            "re-run the review so the recorded sha resolves, or restore the history "
            "(a shallow clone cannot answer this)",
        )
        continue

    changed = subprocess.run(
        ["git", "-C", str(root), "diff", "--name-only", f"{base}..HEAD"],
        capture_output=True,
        text=True,
    )
    if changed.returncode != 0:
        continue

    prefix = f"{rel_dir}/"
    stale = sorted(
        p
        for p in changed.stdout.split("\n")
        if p and p.startswith(prefix) and is_durable_contract(p[len(prefix):])
    )
    if stale:
        shown = ", ".join(stale[:3])
        more = f" (+{len(stale) - 3} more)" if len(stale) > 3 else ""
        emit(
            rel_dir,
            f"done spec's review is stale — {len(stale)} durable contract(s) changed "
            f"since reviewed-against {base[:8]}: {shown}{more}",
            f"re-run review against {feature} before releasing",
        )

sys.exit(1 if findings else 0)
PY
exit $?
