#!/usr/bin/env bash
# scripts/audit/manifest-destination-links.sh — Family 35 of /audit.
#
# A relative markdown link in a file the installer SHIPS, resolved against the
# path that file occupies in the ADOPTER's tree rather than the path it
# occupies here.
#
# Every existing link check resolves against this repository, where a shipped
# file's relative links are correct by construction — that is why they were
# written that way. `check-corpus-links` walks the spec corpus of whatever repo
# it runs in; Family 26 walks this one. Neither can see that
# `framework/rules/quality-cross.md`'s `../../specs/017-…` link, correct at
# `framework/rules/`, becomes `<adopter>/specs/017-…` once the manifest copies
# the file to `specs/rules/`. The adopter has no spec 017 — ductus's `specs/`
# is its own development record and is not in the manifest — so the link is
# dead the moment it lands, and every check in the suite passes.
#
# It was not caught here. It was reported from an adopter run, where the
# release that shipped the dangling link also shipped the `check-corpus-links`
# pre-commit step, which is corpus-wide rather than staged-scoped. The rule
# file installs INSIDE the spec root that step scans, so the same release gave
# the adopter a broken link and a hook that refused every commit — not merely
# spec commits — until they hand-edited a framework-managed file. That is the
# failure this family exists to make impossible: the defect is not that a link
# broke, it is that no check in the repository was looking at the tree where it
# breaks.
#
# THE SUBJECT IS THE MANIFEST, AND IT IS SPLIT IN TWO, DELIBERATELY.
#
#   35a  **Shared Files** — entries whose destinations are literal paths
#        (`specs/rules/*.md`, `.ductus/constitution.md`, `.githooks/*`). Each
#        source is copied to its destination in a throwaway tree and
#        `check-corpus-links --scope repository` is run there. THE CHECK IS THE
#        PRIMITIVE AND THIS IS THE ENTRY POINT — the Family 30 shape, and here
#        it is load-bearing rather than stylistic: a second link resolver
#        written for the adopter tree would be a second implementation of the
#        rule Family 26 already delegates, and the two would diverge exactly as
#        Family 26's own python copy diverged from the primitive within a day
#        of it shipping. One resolver, a third subject, and the difference is a
#        directory rather than a fork.
#
#   35b  **Slash commands** — entries whose destinations carry placeholders
#        (`{config_dir}/commands/{project}/…`). These CANNOT go through 35a,
#        for two independent reasons, so the check is lexical instead: the
#        destination has no literal form without inventing an agent and a
#        project name, and the primitive excludes the host's config dir by
#        construction — it resolves `Host::load(repo).cli_config_dir` out of
#        the walk precisely so it never reports an adopter's generated command
#        copies, whose links are broken by design. Building a tree the
#        primitive is built to ignore would report a confident zero.
#
#        The lexical rule is exact rather than heuristic, and it is exact
#        because of what the destination directory CONTAINS: every
#        ductus-authored file there is a sibling `.md` from this same manifest.
#        So a relative link whose target holds a path separator leaves that set
#        by definition — `../constitution.md` and `commands/review.md` both do,
#        and both shipped. A bare sibling (`review.md`) resolves and is not a
#        finding. No resolution, no filesystem, no guessing.
#
# WHAT THIS FAMILY DOES NOT ASSERT. 35a proves that a shipped file's links
# resolve against the *ductus-authored* portion of an adopter's tree. It says
# nothing about a link into adopter-owned content — a template naming
# `specs/NNN-feature/spec.md` is a documented shape, not a pointer, and the
# shape filter drops it on both sides. Both counts go to stderr so a clean exit
# reads as "the shipped set resolves" and never as "every link an adopter has
# resolves", which is a claim nothing in this repository can make.
#
# An empty extraction on either manifest table, an unreachable runtime, and an
# unparseable result are each findings. A family that examined nothing must not
# exit like one that examined everything.
#
# Bash 3.2 compatible (macOS system bash). The runtime emits JSON and the 35b
# scan needs fenced-block and code-span stripping, so both compute in python3
# and render tab-separated records the shell feeds to `emit` — the convention
# ./README.md records for the families that compute in python.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family manifest-dest

MANIFEST_FILE="framework/bootstrap/ductus.md"
SELF="scripts/audit/manifest-destination-links.sh"

# Extract `source<TAB>dest` rows from a manifest table. The section heading is
# the anchor; `### ` subheadings do not match `^## `, so a `## `-delimited
# range holds every table under the section.
manifest_pairs() {
  sed -n "$1" "$MANIFEST_FILE" 2> /dev/null \
    | grep -E '^\| `' \
    | sed -E 's/^\| `([^`]+)` *\| `([^`]+)` *\|.*/\1'$'\t''\2/'
}

# ---------------------------------------------------------------------------
# 35a — Shared Files, resolved in a synthetic adopter tree
# ---------------------------------------------------------------------------

shared_pairs="$(manifest_pairs '/^## Shared Files$/,/^## /p')"
shared_count="$(printf '%s\n' "$shared_pairs" | grep -c '[^[:space:]]')"
examined_a=0
excluded_a=0

if [ "$shared_count" -eq 0 ]; then
  emit "$MANIFEST_FILE" \
    "Shared Files manifest yielded no source/destination pairs — nothing to resolve against an adopter tree" \
    "restore the '## Shared Files' section's tables, or update this family's extraction if the table shape changed"
else
  bin="$(ductus_bin)"
  if [ -z "$bin" ]; then
    emit "$SELF" \
      "no ductus runtime reachable — adopter-tree link resolution did not run" \
      "build the runtime (cargo build --release --manifest-path runtime/Cargo.toml) or run /ductus to acquire the pinned binary"
  else
    bin="$(cd "$(dirname "$bin")" && pwd)/$(basename "$bin")"
    synth="$(mktemp -d)"
    trap 'rm -rf "$synth"' EXIT

    placed=0
    while IFS=$'\t' read -r src dst; do
      [ -z "$src" ] && continue
      if [ ! -f "$src" ]; then
        emit "$MANIFEST_FILE" \
          "Shared Files manifest names a source that does not exist: $src" \
          "correct the manifest row, or restore the file it ships"
        continue
      fi
      mkdir -p "$synth/$(dirname "$dst")"
      cp "$src" "$synth/$dst"
      placed=$((placed + 1))
    done <<< "$shared_pairs"

    if [ "$placed" -eq 0 ]; then
      emit "$MANIFEST_FILE" \
        "no Shared Files source could be placed into the synthetic adopter tree — the resolution never ran" \
        "check the manifest's source paths against the working tree"
    else
      # `--scope repository` reads `git ls-files`, so the throwaway tree has to
      # be one. Staged-not-committed is enough, and avoids needing a committer
      # identity that a maintainer's environment may not supply.
      git -C "$synth" init -q > /dev/null 2>&1
      git -C "$synth" add -A > /dev/null 2>&1

      report="$synth/.audit-report"
      (cd "$synth" && "$bin" check-corpus-links --scope repository 2> /dev/null) \
        | python3 -c '
import json, sys
raw = sys.stdin.read()
start = raw.find("{")
if start < 0:
    sys.exit(1)
try:
    data = json.loads(raw[start:])
except ValueError:
    sys.exit(1)
for item in data.get("broken", []):
    print("broken\t%s\t%s\t%s" % (item.get("path", ""), item.get("line", ""), item.get("target", "")))
for item in data.get("skipped", []):
    print("skipped\t%s\t%s\t" % (item.get("path", ""), item.get("reason", "")))
if data.get("guidance"):
    print("guidance\t%s\t\t" % data["guidance"])
print("counts\t%d\t%d\t" % (len(data.get("examined", [])), data.get("excluded-by-construction", 0)))
' > "$report" 2> /dev/null

      if [ ! -s "$report" ]; then
        emit "$SELF" \
          "check-corpus-links produced no parseable result for the synthetic adopter tree" \
          "run the primitive by hand in a tree built from the Shared Files manifest and inspect its output"
      else
        while IFS=$'\t' read -r kind f1 f2 f3; do
          case "$kind" in
            broken)
              emit "$f1:$f2" \
                "shipped file links \`$f3\`, which resolves in this repo but not from its manifest destination — an adopter's copy dangles" \
                "point it at an absolute https://github.com/stonean/ductus/blob/main/… URL when the target never ships, or name it in prose when it does"
              ;;
            skipped)
              emit "$f1" \
                "shipped file could not be read in the synthetic adopter tree ($f2) — its links were not examined" \
                "resolve the read failure; an unexaminable file must not be counted as clean"
              ;;
            guidance)
              emit "$SELF" \
                "check-corpus-links could not establish its subject in the synthetic tree: $f1" \
                "inspect the synthetic tree construction above; a subject that could not be listed is not an empty one"
              ;;
            counts)
              examined_a="$f1"
              excluded_a="$f2"
              ;;
          esac
        done < "$report"
      fi
    fi
  fi
fi

# ---------------------------------------------------------------------------
# 35b — Slash commands, checked lexically
# ---------------------------------------------------------------------------

command_pairs="$(manifest_pairs '/^### Slash commands (strategy: update)$/,/^### /p')"
command_count="$(printf '%s\n' "$command_pairs" | grep -c '[^[:space:]]')"
examined_b=0

if [ "$command_count" -eq 0 ]; then
  emit "$MANIFEST_FILE" \
    "slash-command manifest yielded no source/destination pairs — no command source was examined" \
    "restore the '### Slash commands (strategy: update)' table, or update this family's extraction if the table shape changed"
else
  sources="$(printf '%s\n' "$command_pairs" | cut -f1 | grep -v '{' | sort -u)"
  scanned="$(printf '%s\n' "$sources" | grep -c '[^[:space:]]')"
  if [ "$scanned" -eq 0 ]; then
    emit "$MANIFEST_FILE" \
      "every slash-command manifest source carries a placeholder — no literal command source was examined" \
      "check the table's Source Path column; only the configure row is expected to be templated"
  else
    findings="$(printf '%s\n' "$sources" | python3 -c '
import re, sys

LINK = re.compile(r"\[[^\]]*\]\(([^)\s]+)")
SPAN = re.compile(r"`[^`]*`")
ABSOLUTE = ("http://", "https://", "mailto:", "#")

count = 0
for path in (line.strip() for line in sys.stdin):
    if not path:
        continue
    try:
        with open(path, encoding="utf-8") as handle:
            lines = handle.read().splitlines()
    except OSError as err:
        print("unreadable\t%s\t\t%s" % (path, err.strerror))
        continue
    count += 1
    fenced = False
    for number, line in enumerate(lines, 1):
        if line.lstrip().startswith("```"):
            fenced = not fenced
            continue
        if fenced:
            continue
        # Code spans are documentation of a shape, never a live pointer; the
        # primitive drops them for the same reason and this must agree.
        for target in LINK.findall(SPAN.sub("", line)):
            if target.startswith(ABSOLUTE):
                continue
            if "{" in target or "*" in target or "NNN" in target:
                continue
            if "/" in target:
                print("escapes\t%s\t%d\t%s" % (path, number, target))
print("scanned\t%d\t\t" % count)
')"
    while IFS=$'\t' read -r kind f1 f2 f3; do
      case "$kind" in
        escapes)
          emit "$f1:$f2" \
            "command source links \`$f3\`, which leaves its installed directory — an adopter's copy sits in {config_dir}/commands/{project}/, where only sibling command files exist" \
            "point it at an absolute https://github.com/stonean/ductus/blob/main/… URL when the target never ships, or name it in prose when it does"
          ;;
        unreadable)
          emit "$f1" \
            "slash-command manifest source could not be read ($f3) — its links were not examined" \
            "resolve the read failure; an unexaminable source must not be counted as clean"
          ;;
        scanned)
          examined_b="$f1"
          ;;
      esac
    done <<< "$findings"

    if [ "$examined_b" -eq 0 ]; then
      emit "$MANIFEST_FILE" \
        "no slash-command source could be read — the lexical pass examined nothing" \
        "check the table's source paths against the working tree"
    fi
  fi
fi

# The counts are the guard, on stderr, in the shape Families 26 and 34 use: a
# clean exit means *these* subjects resolved, and a future narrowing of either
# one is visible here rather than inferred from silence.
echo "manifest-dest: 35a — ${examined_a} shipped file(s) examined in a synthetic adopter tree built from ${shared_count} Shared Files manifest entr(ies); ${excluded_a} excluded by construction" >&2
echo "manifest-dest: 35b — ${examined_b} slash-command source(s) scanned lexically from ${command_count} manifest entr(ies); the installed copies' directory is excluded from 35a by the primitive's design" >&2

exit "$drift"
