#!/usr/bin/env bash
# scripts/audit/marker-list-parity.sh — Family 18 of /audit.
#
# Binds the `criterion-path-existence` non-assertion marker list — and the
# negated-creation predicate's word lists beside it — to their canonical
# source, so the three restatements of each cannot drift from it.
#
# The list decides when an acceptance criterion is *not* claiming its paths
# are present — a deletion, a rename, a migration subject, an adopter-scoped
# path, a hedge — and so decides which findings the family suppresses. It
# lives in four places by necessity:
#
#   * `specs/045-decision-state-drift-detection/data-model.md` — the canonical
#     source, per the constitution's canonical-sources map;
#   * `runtime/src/primitives/check_artifacts.rs` — `NON_ASSERTION_MARKERS`,
#     the implementation;
#   * `framework/commands/analyze.md` — the adopter-facing restatement. This
#     one cannot be replaced by a pointer: `analyze.md` ships to adopter
#     projects, which have no copy of 045's data-model;
#   * `specs/067-analyze-and-audit-checks/scenarios/criterion-path-existence-family.md`
#     — carries the count and the group names, not the phrases.
#
# Nothing compared them. Adding three phrases (022 scenario
# `criterion-non-assertion-phrasings`) meant hand-editing all four, and a
# missed one would have left a canonical source lying about shipped
# behaviour — the exact drift §drift-prevention exists to catch, in the
# check built to catch it. Surfaced as a QUAL-GROUND-001 SHOULD by
# `/ductus:review` on 022, 2026-08-03.
#
# Method:
#   18a Derive the marker set from the canonical table in 045's data-model.
#       A failed or empty derivation is a finding, never a silent pass —
#       the same fail-closed direction Family 17 takes.
#   18b Parse `NON_ASSERTION_MARKERS` out of `check_artifacts.rs` and compare
#       as a set, plus assert the declared array length matches.
#   18c Parse `analyze.md`'s restatement and compare as a set.
#   18d Compare the spelled-out counts in both markdown restatements against
#       the derived size.
#   18e Do 18a-18c again for the negated-creation predicate's vocabulary —
#       the sixth exemption group, which is a clause-scoped predicate rather
#       than a phrase, and so carries two closed word lists of its own in the
#       same three places. No count arm: its prose states no count, which is
#       deliberate (a count that is never written cannot go stale).
#
# Table convention: a phrase whose trailing space is significant is written
# `` `text` + space `` in markdown, because a trailing space inside an inline
# code span trips markdownlint MD038. The derivation reverses that here, in
# one place, so the documents stay lint-clean and the comparison stays exact.
#
# Deliberately NOT a finding:
#   - Group names or row ordering. The check is on the phrase *set*; the
#     grouping is editorial and readers, not code, consume it. 18e compares
#     the predicate's two word lists as one union for the same reason: a
#     negator swapped into the verb row is not a drift mode anyone has, while
#     a word dropped from either list is, and the union catches that.
#   - The 022 scenario's phrase-free prose. It states counts and group names
#     only, and its count is covered by 18d.
#
# Requires `python3`.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family marker-list-parity

if ! command -v python3 >/dev/null 2>&1; then
  emit "(precondition)" "python3 not on PATH — cannot parse the marker lists" \
    "install python3 and re-run"
  exit 1
fi

CANON_MD="$ROOT/specs/045-decision-state-drift-detection/data-model.md"
CHECK_RS="$ROOT/runtime/src/primitives/check_artifacts.rs"
ANALYZE_MD="$ROOT/framework/commands/analyze.md"
SCENARIO_MD="$ROOT/specs/067-analyze-and-audit-checks/scenarios/criterion-path-existence-family.md"

for f in "$CANON_MD" "$CHECK_RS" "$ANALYZE_MD" "$SCENARIO_MD"; do
  if [ ! -f "$f" ]; then
    emit "${f#"$ROOT"/}" "file named by this family is missing — cannot compare the marker list" \
      "restore the file, or update scripts/audit/marker-list-parity.sh if it moved"
    exit 1
  fi
done

python3 - "$CANON_MD" "$CHECK_RS" "$ANALYZE_MD" "$SCENARIO_MD" <<'PY'
import re
import sys

canon_md, check_rs, analyze_md, scenario_md = sys.argv[1:5]

NUMBER_WORDS = {
    n: w
    for n, w in enumerate(
        "zero one two three four five six seven eight nine ten eleven twelve "
        "thirteen fourteen fifteen sixteen seventeen eighteen nineteen twenty".split(),
    )
}

findings = []


def emit(location, message, fix):
    findings.append(f"marker-list-parity | {location} | {message} | {fix}")


def spans(cell):
    """Code spans in a table cell, applying the `+ space` convention."""
    out = []
    for raw, suffix in re.findall(r"`([^`]+)`(\s*\+ space)?", cell):
        out.append(raw + " " if suffix else raw)
    return out


def rust_literals(body):
    """String literals in a Rust array body, comment lines stripped first.

    Stripping is what keeps a phrase quoted in a `//` comment from being read
    as an entry. Shared by 18b and 18e, which parse the same array shape.
    """
    uncommented = "\n".join(
        line for line in body.splitlines() if not line.strip().startswith("//")
    )
    return re.findall(r'"((?:[^"\\]|\\.)*)"', uncommented)


# --- 18a: derive from the canonical table -----------------------------------

text = open(canon_md, encoding="utf-8").read()
section = re.search(
    r"### The criterion must be a live claim\n(.*?)(?=\n### |\n## |\Z)",
    text,
    re.S,
)
canonical = []
if section:
    for line in section.group(1).splitlines():
        if not line.startswith("|"):
            continue
        cols = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cols) < 2 or cols[0] in ("Group", "---") or set(cols[0]) <= {"-", " "}:
            continue
        canonical.extend(spans(cols[1]))

if not canonical:
    emit(
        "specs/045-decision-state-drift-detection/data-model.md",
        "derived zero markers from the canonical table under "
        "'### The criterion must be a live claim' — the derivation is broken "
        "or the table moved, so this family would pass while checking nothing",
        "restore the table, or update the derivation in "
        "scripts/audit/marker-list-parity.sh to match its new shape",
    )
    print("\n".join(findings))
    sys.exit(1)

canonical_set = set(canonical)
if len(canonical_set) != len(canonical):
    dupes = sorted({m for m in canonical if canonical.count(m) > 1})
    emit(
        "specs/045-decision-state-drift-detection/data-model.md",
        f"canonical table lists duplicate marker(s): {', '.join(repr(d) for d in dupes)}",
        "remove the duplicate row(s) so the canonical set is unambiguous",
    )

# --- 18b: the Rust implementation -------------------------------------------

rs = open(check_rs, encoding="utf-8").read()
decl = re.search(
    r"const NON_ASSERTION_MARKERS: \[&str; (\d+)\] = \[(.*?)\n\];",
    rs,
    re.S,
)
if not decl:
    emit(
        "runtime/src/primitives/check_artifacts.rs",
        "could not locate the `NON_ASSERTION_MARKERS` declaration",
        "keep the `const NON_ASSERTION_MARKERS: [&str; N] = [ … ];` shape, "
        "or update scripts/audit/marker-list-parity.sh",
    )
else:
    declared_len = int(decl.group(1))
    rust = rust_literals(decl.group(2))
    rust_set = set(rust)
    if declared_len != len(rust):
        emit(
            "runtime/src/primitives/check_artifacts.rs",
            f"array declares length {declared_len} but holds {len(rust)} literals",
            f"set the declared length to {len(rust)}",
        )
    missing = canonical_set - rust_set
    extra = rust_set - canonical_set
    if missing:
        emit(
            "runtime/src/primitives/check_artifacts.rs",
            "canonical marker(s) absent from NON_ASSERTION_MARKERS: "
            + ", ".join(repr(m) for m in sorted(missing)),
            "add them to the array, or drop the row(s) from 045's data-model "
            "if the marker was retired",
        )
    if extra:
        emit(
            "runtime/src/primitives/check_artifacts.rs",
            "NON_ASSERTION_MARKERS holds marker(s) the canonical table does not list: "
            + ", ".join(repr(m) for m in sorted(extra)),
            "add the row(s) to the table in "
            "specs/045-decision-state-drift-detection/data-model.md, "
            "or remove them from the array",
        )

# --- 18c: the adopter-facing restatement ------------------------------------

analyze = open(analyze_md, encoding="utf-8").read()
sentence = re.search(
    r"A criterion carrying any of \w+ closed phrases is exempted whole[^\n]*",
    analyze,
)
if not sentence:
    emit(
        "framework/commands/analyze.md",
        "could not locate the non-assertion marker restatement",
        "keep the 'A criterion carrying any of <count> closed phrases is "
        "exempted whole' sentence, or update the audit",
    )
else:
    # Only parenthesised groups that are *entirely* comma-separated code
    # spans count. That structural filter separates the marker groups —
    # `(`deleted`, `does not exist`, …)` — from prose parentheticals and
    # from code spans elsewhere in the sentence (the `not-a-live-claim`
    # reason name is one), without having to guess where the sentence ends.
    # `e.g.` carries dots, so sentence-splitting is not an option here.
    analyze_set = set()
    # The alternation lets a code span carry its own parens — the `(was`
    # marker is exactly that case — while still stopping at the group's
    # own closing paren.
    for group in re.findall(r"\(((?:[^()`]|`[^`]*`)*)\)", sentence.group(0)):
        parts = [p.strip() for p in group.split(",")]
        if not parts or not all(
            re.fullmatch(r"`[^`]+`(\s*\+ space)?", p) for p in parts
        ):
            continue
        analyze_set.update(spans(group))
    missing = canonical_set - analyze_set
    extra = analyze_set - canonical_set
    if missing:
        emit(
            "framework/commands/analyze.md",
            "canonical marker(s) missing from the shipped restatement: "
            + ", ".join(repr(m) for m in sorted(missing)),
            "add them — adopters have no copy of 045's data-model, so this "
            "restatement is their only view of the list",
        )
    if extra:
        emit(
            "framework/commands/analyze.md",
            "restatement lists marker(s) the canonical table does not: "
            + ", ".join(repr(m) for m in sorted(extra)),
            "reconcile against specs/045-decision-state-drift-detection/data-model.md",
        )

# --- 18d: the spelled-out counts --------------------------------------------

expected_word = NUMBER_WORDS.get(len(canonical_set))
for path, label, pattern in (
    (canon_md, "specs/045-decision-state-drift-detection/data-model.md",
     r"any of these (\w+) phrases is exempted whole"),
    (analyze_md, "framework/commands/analyze.md",
     r"carrying any of (\w+) closed phrases is exempted whole"),
    (scenario_md, "specs/067-analyze-and-audit-checks/scenarios/criterion-path-existence-family.md",
     r"(\w+) closed phrases mark a criterion"),
):
    body = open(path, encoding="utf-8").read()
    found = re.search(pattern, body)
    if not found:
        emit(label, "could not locate the spelled-out marker count",
             "keep the count sentence, or update the audit's pattern")
        continue
    stated = found.group(1).lower()
    if expected_word is None:
        emit(label,
             f"canonical set has {len(canonical_set)} markers, beyond the audit's "
             "number-word table",
             "extend NUMBER_WORDS in scripts/audit/marker-list-parity.sh")
    elif stated != expected_word:
        emit(label,
             f"states '{stated}' markers but the canonical table lists "
             f"{len(canonical_set)} ({expected_word})",
             f"change '{stated}' to '{expected_word}'")

# --- 18e: the negated-creation predicate's vocabulary -----------------------
#
# The sixth exemption group is a predicate, not a phrase — its negator and its
# verb are separated by the noun between them, so no fixed phrase spans the
# construction. It therefore carries two closed word lists instead of a table
# row, in the same three places, with the same consequence if one drifts.

predicate = re.search(
    r"### A criterion asserting a path was never created\n(.*?)(?=\n### |\n## |\Z)",
    text,
    re.S,
)
canonical_words = []
if predicate:
    for line in predicate.group(1).splitlines():
        if not line.startswith("|"):
            continue
        cols = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cols) < 2 or cols[0] in ("Role", "---") or set(cols[0]) <= {"-", " "}:
            continue
        canonical_words.extend(spans(cols[1]))

if not canonical_words:
    # Fail closed, exactly as 18a does: a derivation that yields nothing must
    # never read as agreement.
    emit(
        "specs/045-decision-state-drift-detection/data-model.md",
        "derived zero words from the table under '### A criterion asserting a "
        "path was never created' — the derivation is broken or the section "
        "moved, so this arm would pass while checking nothing",
        "restore the table, or update the derivation in "
        "scripts/audit/marker-list-parity.sh to match its new shape",
    )
else:
    canonical_words_set = set(canonical_words)

    rust_words = set()
    for name in ("CREATION_NEGATORS", "CREATION_VERBS"):
        found = re.search(rf"const {name}: \[&str; (\d+)\] = \[(.*?)\];", rs, re.S)
        if not found:
            emit(
                "runtime/src/primitives/check_artifacts.rs",
                f"could not locate the `{name}` declaration",
                f"keep the `const {name}: [&str; N] = [ … ];` shape, "
                "or update scripts/audit/marker-list-parity.sh",
            )
            continue
        literals = rust_literals(found.group(2))
        if int(found.group(1)) != len(literals):
            emit(
                "runtime/src/primitives/check_artifacts.rs",
                f"`{name}` declares length {found.group(1)} but holds "
                f"{len(literals)} literals",
                f"set the declared length to {len(literals)}",
            )
        rust_words.update(literals)

    bullet = re.search(
        r"A criterion asserting a path was never created is exempted the same way[^\n]*",
        analyze,
    )
    analyze_words = set()
    if not bullet:
        emit(
            "framework/commands/analyze.md",
            "could not locate the negated-creation restatement",
            "keep the 'A criterion asserting a path was never created is "
            "exempted the same way' bullet, or update the audit",
        )
    else:
        # Same structural filter as 18c: only parenthesised groups that are
        # entirely comma-separated code spans count, which separates the two
        # word lists from every prose parenthetical around them.
        for group in re.findall(r"\(((?:[^()`]|`[^`]*`)*)\)", bullet.group(0)):
            parts = [p.strip() for p in group.split(",")]
            if not parts or not all(
                re.fullmatch(r"`[^`]+`(\s*\+ space)?", p) for p in parts
            ):
                continue
            analyze_words.update(spans(group))

    for label, got, fix in (
        (
            "runtime/src/primitives/check_artifacts.rs",
            rust_words,
            "reconcile CREATION_NEGATORS / CREATION_VERBS against the table in "
            "specs/045-decision-state-drift-detection/data-model.md",
        ),
        (
            "framework/commands/analyze.md",
            analyze_words,
            "reconcile the negated-creation bullet against "
            "specs/045-decision-state-drift-detection/data-model.md — adopters "
            "have no copy of it, so this restatement is their only view",
        ),
    ):
        if not got:
            # Fail closed, the same direction 18a takes. A restatement that
            # still exists but no longer parses is exactly the shape that
            # would otherwise read as agreement.
            emit(label,
                 "parsed zero negated-creation words from this restatement — "
                 "it is missing or no longer matches the shape the audit reads",
                 fix)
            continue
        missing = canonical_words_set - got
        extra = got - canonical_words_set
        if missing:
            emit(label,
                 "canonical negated-creation word(s) absent: "
                 + ", ".join(repr(w) for w in sorted(missing)), fix)
        if extra:
            emit(label,
                 "negated-creation word(s) the canonical table does not list: "
                 + ", ".join(repr(w) for w in sorted(extra)), fix)

if findings:
    print("\n".join(findings))
    sys.exit(1)
sys.exit(0)
PY
exit $?
