#!/usr/bin/env bash
# scripts/audit/shipped-anchor-resolution.sh — Family 39 of /audit.
#
# Every `§` reference in a markdown file under framework/ resolves under
# `resolve-anchor` against framework/constitution.md.
#
# An adopter ran that check by hand over its installed
# `.claude/commands/ductus.md` against `.ductus/constitution.md`, because a
# decision it had stored named the check, and got six unresolved anchors it
# could not fix: the file is framework-managed, so the only repair is a ductus
# release. At the time the same file carried seventeen here, one of them added
# by the release two days earlier, and every family in this suite was green.
# `/{project}:analyze` runs `resolve-anchor` over the targeted spec alone, and
# nothing in this repository ran it over what ductus ships. That is the gap:
# the check existed, adopters could run it, and the only place its findings
# could be repaired never asked.
#
# THE SUBJECT IS framework/, WHOLE, ENUMERATED FROM GIT. Every file under it
# ships in the release archive a `/ductus` run extracts; the installed ones are
# a subset. Enumerating from `git ls-files` rather than from the manifest keeps
# the archive-half files in scope — `ductus-procedure.md` and the migration
# bodies, which a host reads during a run without installing — and they carried
# twenty of the references that did not resolve. `govern.md` is included though
# Family 21 holds it byte-identical to `ductus.md`: pre-rename installs still
# fetch it, so it ships in its own right.
#
# THE CHECK IS THE PRIMITIVE AND THIS IS THE ENTRY POINT — the Family 30 shape.
# A second anchor resolver here would drift from the one an adopter runs, and
# the report this family answers is about that one. So its limits are the
# primitive's, inherited rather than restated as policy: another document is
# recognised only by a `.md` filename or link on the same line, a line naming
# one qualifies every `§` on it, and a reference meant for another file can
# resolve against a same-named local heading. The scenario states each.
#
# Only `markers`- and `cross-document`-kind references can come back
# unresolved; `qualified` and `intra-document` are counted, never judged,
# exactly as the primitive reports them. `cross-document` (spec 071) is a
# command step's pointer into its reference file, **§X** in
# `.ductus/reference/<command>.md`: the line names one document the Shared
# Files manifest ships, so the primitive judges it against that document's
# framework source. The counts go to stderr so a clean exit says what it
# examined.
#
# An empty enumeration, an unreachable runtime, and a file whose result could
# not be read or parsed are each findings. A family that examined nothing must
# not exit like one that examined everything.
#
# Bash 3.2 compatible (macOS system bash). The runtime emits JSON, so the loop
# computes in python3 and renders tab-separated records the shell feeds to
# `emit` — the convention ./README.md records for the families that compute in
# python.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family shipped-anchors

MARKERS="framework/constitution.md"
SELF="scripts/audit/shipped-anchor-resolution.sh"

# git's pathspec `*` crosses `/`, so this one pattern reaches every depth.
files="$(git ls-files 'framework/*.md' 2> /dev/null)"
count="$(printf '%s\n' "$files" | grep -c '[^[:space:]]')"
examined=0
refs=0
qualified=0
intra=0
marker_refs=0
cross=0

if [ "$count" -eq 0 ]; then
  emit "framework/" \
    "no tracked markdown file under framework/ was enumerated — no § reference was resolved" \
    "confirm git is available and this is a ductus checkout; an empty subject is not a clean one"
elif [ ! -f "$MARKERS" ]; then
  emit "$MARKERS" \
    "the constitution is missing, so no § reference has markers to resolve against" \
    "restore $MARKERS"
else
  bin="$(ductus_bin)"
  if [ -z "$bin" ]; then
    emit "$SELF" \
      "no ductus runtime reachable — anchor resolution over framework/ did not run" \
      "build the runtime (cargo build --release --manifest-path runtime/Cargo.toml) or run /ductus to acquire the pinned binary"
  else
    records="$(printf '%s\n' "$files" | python3 -c '
import json, subprocess, sys

binary, markers = sys.argv[1], sys.argv[2]
examined = refs = qualified = intra = marker_refs = cross = 0
for path in (line.strip() for line in sys.stdin):
    if not path:
        continue
    try:
        run = subprocess.run(
            [binary, "resolve-anchor", "--path", path, "--markers-path", markers],
            capture_output=True, text=True, timeout=60,
        )
    except (OSError, subprocess.TimeoutExpired) as err:
        print("unexamined\t%s\t%s" % (path, type(err).__name__))
        continue
    if run.returncode != 0:
        lines = run.stderr.strip().splitlines() or ["exit %d" % run.returncode]
        print("unexamined\t%s\t%s" % (path, lines[-1].replace("\t", " ")))
        continue
    try:
        data = json.loads(run.stdout[run.stdout.find("{"):])
        references = data["references"]
    except (ValueError, KeyError, TypeError):
        print("unexamined\t%s\tunparseable result" % path)
        continue
    examined += 1
    for ref in references:
        refs += 1
        kind = ref.get("kind")
        if kind == "qualified":
            qualified += 1
        elif kind == "intra-document":
            intra += 1
        elif kind == "cross-document":
            cross += 1
            if not ref.get("resolved"):
                print("unresolved-cross\t%s:%s\t%s" % (path, ref.get("line", ""), ref.get("anchor", "")))
        else:
            marker_refs += 1
            if not ref.get("resolved"):
                print("unresolved\t%s:%s\t%s" % (path, ref.get("line", ""), ref.get("anchor", "")))
print("counts\t%d\t%d\t%d\t%d\t%d\t%d" % (examined, refs, qualified, intra, marker_refs, cross))
' "$bin" "$MARKERS")"

    while IFS=$'\t' read -r kind f1 f2 f3 f4 f5 f6; do
      case "$kind" in
        unresolved)
          emit "$f1" \
            "\`§$f2\` names no heading in this file and no marker in $MARKERS — resolve-anchor reports it to anyone who runs the check over what ductus ships, and only a ductus release repairs it" \
            "spell a local heading as it reads, name the other markdown document on the same line, or drop the § for a section of a non-markdown document"
          ;;
        unresolved-cross)
          emit "$f1" \
            "\`§$f2\` names no heading of the shipped document this line points into, no heading in this file, and no marker in $MARKERS" \
            "spell the target section's heading in full, as it reads in the framework source the Shared Files manifest maps that document to"
          ;;
        unexamined)
          emit "$f1" \
            "resolve-anchor produced no usable result for this file ($f2) — its § references were not examined" \
            "run '$bin resolve-anchor --path $f1 --markers-path $MARKERS' by hand and resolve the failure"
          ;;
        counts)
          examined="$f1"
          refs="$f2"
          qualified="$f3"
          intra="$f4"
          marker_refs="$f5"
          cross="$f6"
          ;;
      esac
    done <<< "$records"

    if [ "$examined" -eq 0 ]; then
      emit "$SELF" \
        "resolve-anchor examined none of the $count enumerated file(s) — the subject was not checked, not clean" \
        "run the primitive by hand over one framework file and inspect its output"
    fi
  fi
fi

echo "shipped-anchors: ${examined} of ${count} framework/ markdown file(s) examined; ${refs} § reference(s) — ${marker_refs} against constitution markers, ${cross} cross-document (judged against a shipped document's source), ${intra} intra-document, ${qualified} qualified (excluded by construction)" >&2

exit "$drift"
