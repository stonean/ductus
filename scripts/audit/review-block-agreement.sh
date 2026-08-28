#!/usr/bin/env bash
# scripts/audit/review-block-agreement.sh — Family 31 of /audit.
#
# A reviewed spec records the same review twice. `spec.md`'s frontmatter carries
# a `review:` block — last-run, reviewed-against, must-violations,
# should-violations, low-confidence, blocking, waivers — and the `review.md`
# written alongside it carries its own frontmatter with reviewed-at,
# reviewed-against, and the same three counts. Nothing held the two together.
#
# They drifted, and for weeks. 031 and 041 both carried `should-violations: 1`
# in `spec.md` while their own `review.md` recorded `0`. The drift was invisible
# because every gate reads exactly one of the two files and never the other:
# `check-review-gate` and `/ductus:analyze`'s review-drift check read the
# `spec.md` block; Family 19 (review-freshness) resolves `reviewed-against` to
# decide whether a review predates its code, but never compares the counts on
# either side of it.
#
# The root cause in the 031 case was a waiver moved into `review.md` by hand
# with no matching `review.waivers` entry in `spec.md`. The waived finding had
# no structural existence on the gate's side, so the count it was supposed to
# retire never dropped.
#
# The cost runs both ways. A stale non-zero count reads as outstanding review
# work that does not exist — the signal that sent a maintainer back to re-derive
# two clean specs before a release tag. A stale zero is worse: it would hide
# real findings from `check-review-gate`, `/ductus:analyze`, and the
# `in-progress → done` transition they gate, every one of them trusting a number
# the report beside it contradicts.
#
# This is the shape Families 2, 18, 23, and 28 already exist for — a pair of
# places recording one fact with nothing binding them. The review state is that
# pair, and it is the one pair whose divergence a release gate acts on.
#
# METHOD. Both records are derived from the frontmatter on disk; nothing about
# the expected values is hardcoded, which would be a third copy of the fact
# under test.
#
#   31a The five paired fields, keyed by meaning rather than by key name since
#       the two files spell the timestamp differently:
#
#           spec.md `review:`      review.md
#           last-run           <->  reviewed-at
#           reviewed-against   <->  reviewed-against
#           must-violations    <->  must-violations
#           should-violations  <->  should-violations
#           low-confidence     <->  low-confidence
#
#       Each mismatch names the spec, the field, and both values, so the stale
#       side is visible without opening either file. The fix names `review.md`
#       as the source of record — it is what `/ductus:review` writes from the
#       pass it just ran, while the `spec.md` block is the summary copied
#       forward for the gates — so a disagreement is repaired by re-deriving the
#       block, never by editing the report to match it.
#
#   31b `blocking` agrees with `must-violations`. `blocking: false` alongside a
#       non-zero `must-violations` claims a spec may advance while its own
#       record says it may not. Only that direction is asserted: the reverse
#       (blocking with no violations) is a stuck gate, which surfaces loudly on
#       its own rather than silently letting a spec through.
#
#   31c A waiver has structural existence on both sides. A finding under
#       `review.md`'s `## Waived findings` whose rule has no entry in the spec's
#       `review.waivers` is the 031 root cause exactly: invisible to every gate,
#       and the count it should have retired never moves. Matched on rule id
#       alone — the report renders a file with a line range while the waiver
#       anchors a bare path, and a spurious finding here would send a maintainer
#       editing a waiver that is already correct.
#
# Fields on only one side are not compared: `diff-base`, `captured-issues`, and
# `skipped-passes` in review.md; `blocking` and `waivers` in spec.md. They are
# not duplicated facts, and demanding they match would invent a binding the
# artifacts never claimed.
#
# SUBJECT. The intersection: specs where both records exist and can therefore
# disagree. A `review:` block with no `review.md`, or a `review.md` with no
# block, is a different defect with a different repair, and Family 19 and
# `check-review-gate` already have opinions about the first. Malformed or absent
# frontmatter on a `review.md` that exists is a finding rather than a skip —
# silently dropping a spec from the subject set is how the empty-set false green
# begins.
#
# An empty subject set is a finding, not a pass: a parity check comparing
# nothing reports agreement, which is the precise false green /audit exists to
# prevent.
#
# 026's own spec and review are in the subject set like every other spec's. A
# check that skipped its own spec would be blind to the one divergence it
# authored.
#
# Requires `python3`.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family review-block-agreement

if ! command -v python3 >/dev/null 2>&1; then
  emit "(precondition)" "python3 not on PATH — cannot parse review frontmatter" \
    "install python3 and re-run"
  exit 1
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

python3 - "$ROOT" "$SPECS_ROOT" <<'PY'
import pathlib
import re
import sys

root = pathlib.Path(sys.argv[1])
specs_root = sys.argv[2]
findings = []


def emit(location, message, fix):
    print(f"review-block-agreement | {location} | {message} | {fix}")
    findings.append(location)


def frontmatter(text):
    """The text between the opening and closing `---`, or None when absent."""
    if not text.startswith("---"):
        return None
    end = text.find("\n---", 3)
    return text[3:end] if end != -1 else None


def scalar(fm, key, indent=""):
    # `[ \t]*` never `\s*`: `\s` matches a newline, so a greedy run after the
    # colon walks past an empty value onto the next line and returns *its*
    # content as this key's.
    m = re.search(rf"^{indent}{re.escape(key)}:[ \t]*(.*?)[ \t]*$", fm, re.M)
    if not m:
        return None
    v = m.group(1).strip().strip("\"'")
    return None if v in ("", "null", "~") else v


def review_block(fm):
    """The `review:` mapping's body, or None when the spec carries no block."""
    m = re.search(r"^review:\s*$", fm, re.M)
    if not m:
        return None
    rest = fm[m.end():].lstrip("\n")
    lines = []
    for line in rest.split("\n"):
        if line.strip() and not line.startswith((" ", "\t")):
            break
        lines.append(line)
    return "\n".join(lines)


def waived_rules_in_report(text):
    """Rule ids under `## Waived findings`, from each `### WAIVED: RULE — ...`."""
    m = re.search(r"^##\s+Waived findings\s*$", text, re.M)
    if not m:
        return set()
    rest = text[m.end():]
    nxt = re.search(r"^##\s+", rest, re.M)
    section = rest[: nxt.start()] if nxt else rest
    rules = set()
    for line in re.findall(r"^###\s+WAIVED:\s*(.+?)\s*$", section, re.M):
        # `SIMPLICITY — <summary>` / `BE-AUTHN-001 — <summary>`; the rule is the
        # first token before the em-dash separator.
        rules.add(re.split(r"\s+[—-]\s+", line, maxsplit=1)[0].strip().strip("`"))
    return {r for r in rules if r}


def waived_rules_in_spec(block):
    """Rule ids from the spec's `review.waivers` list entries."""
    if block is None:
        return set()
    m = re.search(r"^[ ]{2}waivers:[ \t]*(.*)$", block, re.M)
    if not m:
        return set()
    if m.group(1).strip() in ("[]", "[ ]"):
        return set()
    rest = block[m.end():]
    rules = set()
    for line in rest.split("\n"):
        if line.strip() and not line.startswith("    "):
            break
        r = re.match(r"^\s*-?\s*rule:\s*(.+?)\s*$", line)
        if r:
            rules.add(r.group(1).strip().strip("\"'`"))
    return {r for r in rules if r}


# Field pairs, keyed by meaning: (spec-side key, report-side key).
PAIRS = [
    ("last-run", "reviewed-at"),
    ("reviewed-against", "reviewed-against"),
    ("must-violations", "must-violations"),
    ("should-violations", "should-violations"),
    ("low-confidence", "low-confidence"),
]

specs_dir = root / specs_root
examined = 0

for spec_path in sorted(specs_dir.glob("*/spec.md")):
    feature = spec_path.parent.name
    report_path = spec_path.parent / "review.md"
    if not report_path.is_file():
        # A `review:` block with no report is Family 19's and
        # check-review-gate's subject, not this family's.
        continue

    rel_spec = f"{specs_root}/{feature}/spec.md"
    rel_report = f"{specs_root}/{feature}/review.md"

    spec_text = spec_path.read_text(encoding="utf-8", errors="replace")
    report_text = report_path.read_text(encoding="utf-8", errors="replace")

    spec_fm = frontmatter(spec_text)
    report_fm = frontmatter(report_text)

    if report_fm is None:
        emit(
            rel_report,
            "review.md has no parseable frontmatter — its record cannot be "
            "compared, which is not the same as agreeing",
            "restore the `---` frontmatter block, or re-run /ductus:review to "
            "regenerate the report",
        )
        continue
    if spec_fm is None:
        emit(
            rel_spec,
            "spec.md has no parseable frontmatter — its review record cannot be "
            "compared, which is not the same as agreeing",
            "restore the `---` frontmatter block on the spec",
        )
        continue

    block = review_block(spec_fm)
    if block is None:
        # A report with no block on the spec side: the gates read a record that
        # is absent, which check-review-gate reports as "not reviewed".
        continue

    examined += 1

    # 31a — the five paired fields.
    for spec_key, report_key in PAIRS:
        spec_value = scalar(block, spec_key, indent="[ ]{2}")
        report_value = scalar(report_fm, report_key)
        if spec_value == report_value:
            continue
        emit(
            rel_spec,
            f"review.{spec_key} is {spec_value!r} but {rel_report}'s "
            f"{report_key} is {report_value!r} — the same review recorded twice, "
            f"disagreeing",
            f"re-derive the spec's `review:` block from {rel_report}, which is "
            f"the source of record; re-run /ductus:review if the report itself "
            f"is out of date",
        )

    # 31b — `blocking` agrees with `must-violations`.
    blocking = (scalar(block, "blocking", indent="[ ]{2}") or "false").lower()
    must = scalar(block, "must-violations", indent="[ ]{2}")
    if blocking == "false" and must is not None and must.isdigit() and int(must) > 0:
        emit(
            rel_spec,
            f"review.blocking is false while review.must-violations is {must} — "
            f"the block says the spec may advance and may not at once",
            "set `blocking: true`, or correct the count if the violations are "
            "resolved or waived, then re-run /ductus:review",
        )

    # 31c — every waived finding has a waiver entry.
    for rule in sorted(waived_rules_in_report(report_text) - waived_rules_in_spec(block)):
        emit(
            rel_spec,
            f"{rel_report} waives {rule} but no `review.waivers` entry records "
            f"it — the waiver has no structural existence, so no gate can see it",
            f"add a `review.waivers` entry for {rule} (rule, file, reason), or "
            f"run /ductus:review --waive {rule} --reason \"...\"",
        )

if examined == 0:
    emit(
        f"{specs_root}/",
        "no spec carries both a `review:` block and a review.md — the "
        "enumeration or the frontmatter parse broke, and comparing nothing "
        "reports agreement",
        f"check that {specs_root}/*/spec.md and review.md are readable and "
        f"carry frontmatter",
    )

print(
    f"review-block-agreement: compared {examined} spec(s) carrying both a "
    f"`review:` block and a review.md",
    file=sys.stderr,
)

sys.exit(1 if findings else 0)
PY
exit $?
