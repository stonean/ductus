#!/usr/bin/env bash
# scripts/audit/broken-relative-links.sh — Family 26 of /audit.
#
# A relative markdown link whose target does not exist. Nothing caught these:
# markdownlint's MD051 validates heading *fragments* and says nothing about
# whether the file exists, and `check-orphaned-references` scopes to
# adopter-owned referrers and ductus-managed path prefixes, so a spec linking a
# sibling spec at the wrong depth is outside both.
#
# The dominant class is a depth error in a scenario file. A scenario lives at
# `specs/NNN-foo/scenarios/bar.md`, so a sibling spec is `../../NNN-other/` and
# the constitution is `../../../framework/`; writing one `../` too few produces
# a link that renders fine, reviews fine, and resolves to nothing. The second
# class is a link to a file a later spec deleted — the same shape as an orphaned
# reference, one directory tier up.
#
# SCOPE, AND WHAT IS EXCLUDED BY CONSTRUCTION. Three categories are skipped
# because a broken link is the *correct* state for them, and each is counted and
# reported rather than silently dropped:
#
#   - `.claude/commands/` — generated output. The generator copies command
#     sources to a different directory depth without rewriting relative links,
#     so the copies' links are broken by construction while the sources' are
#     correct. Auditing the generated tree would report the generator, not a
#     defect, on every run.
#   - `framework/templates/project/` — templates whose links resolve in the
#     *adopter's* repo root after scaffolding, not here.
#   - Documentation shapes — a target naming `NNN`, a `{placeholder}`, or a
#     literal `...`. Prose names link *syntax* as often as it names files.
#
# INLINE CODE SPANS ARE STRIPPED, and that is load-bearing rather than tidiness.
# Docs that discuss linking quote link syntax constantly — `[text](target)`,
# `[constitution.md](constitution.md)`, `[plan](plan.md)` — always inside a code
# span. Without stripping them this family reports 7 false positives on the
# current corpus, every one a doc correctly describing a link rather than making
# one.
#
# A scan that examines no files is a finding, never a pass.
#
# Bash 3.2 compatible; findings computed in python and rendered through `emit`,
# the same split `installer-registry-parity.sh` and `migration-coverage.sh` use,
# so the pipe-separated finding shape never appears in the python.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family broken-link

# The python runs at top level rather than inside `$( ... )`: the source quotes
# backticked paths in its messages, and bash parses a backtick inside command
# substitution as a legacy sub-shell even when the heredoc is quoted. Writing to
# $TMPDIR and reading back sidesteps it — the contract in ./README.md permits
# temp files for intermediate computation.
FINDINGS_FILE="$(mktemp "${TMPDIR:-/tmp}/broken-link.XXXXXX")"
trap 'rm -f "$FINDINGS_FILE"' EXIT

python3 - > "$FINDINGS_FILE" <<'PY'
import os, re, subprocess, sys

try:
    out = subprocess.run(["git", "ls-files", "*.md"], capture_output=True, text=True)
    listing_failed = out.returncode != 0
except OSError:
    out, listing_failed = None, True

if listing_failed:
    # `git` missing or refusing to list. Reported rather than allowed to look
    # like a corpus with no broken links — the whole point of the family is
    # that a silent zero and a real zero must not read alike.
    print("repository\tthe file listing failed, so no link was checked — this is not the same as finding no broken links\tconfirm `git ls-files` works in this checkout; the family cds to the repo root before listing")
    raise SystemExit(0)

files = [f for f in out.stdout.split() if f and not f.startswith("runtime/tests/")]

EXCLUDED_PREFIXES = (".claude/", "framework/templates/project/")
SHAPE = re.compile(r"NNN|[{}]|^\.\.\.$|/\.\.\.$")
LINK = re.compile(r"\[[^\]]*\]\(([^)\s]+)\)")

examined = skipped_generated = skipped_shape = 0
broken = []

for path in files:
    if path.startswith(EXCLUDED_PREFIXES):
        skipped_generated += 1
        continue
    try:
        text = open(path, encoding="utf-8").read()
    except (OSError, UnicodeDecodeError):
        print(f"{path}\tfile could not be read as UTF-8 text, so its links were never checked\tconfirm the file is text; a target this family cannot open is not a target it found clean")
        continue
    examined += 1
    here = os.path.dirname(path)
    # Fences are toggled line by line rather than stripped with a whole-text
    # regex. A regex that deletes the block deletes its newlines too, which
    # shifts every line number after it — findings would point at the wrong
    # line, and the further into the file, the further off.
    fence = False
    for lineno, line in enumerate(text.split("\n"), 1):
        if re.match(r"\s*(```|~~~)", line):
            fence = not fence
            continue
        if fence:
            continue
        # Inline code spans: a doc that *discusses* linking quotes link syntax,
        # and every such quote sits in a span.
        line = re.sub(r"`[^`]*`", "", line)
        for target in LINK.findall(line):
            if target.startswith(("http://", "https://", "mailto:", "#")):
                continue
            rel = target.split("#", 1)[0]
            if not rel:
                continue
            if SHAPE.search(rel):
                skipped_shape += 1
                continue
            if os.path.exists(os.path.normpath(os.path.join(here, rel))):
                continue
            deeper = os.path.normpath(os.path.join(here, "../" + rel))
            hint = (
                f"the target resolves one directory up — write `../{rel}`"
                if os.path.exists(deeper)
                else "confirm the target still exists; if a later spec deleted it, name it in prose instead of linking it"
            )
            broken.append((f"{path}:{lineno}", target, hint))

for where, target, hint in broken:
    print(f"{where}\trelative link `{target}` resolves to nothing\t{hint}")

print(
    f"broken-link: {examined} markdown file(s) examined; "
    f"{skipped_generated} skipped as generated-or-template (links resolve elsewhere by design), "
    f"{skipped_shape} link(s) skipped as documentation shapes",
    file=sys.stderr,
)
if examined == 0:
    print("repository\tno markdown files were examined — the link scan found nothing because it looked at nothing\trun from a git checkout with tracked .md files")
PY

while IFS="$(printf '\t')" read -r where message fix; do
  [ -z "$where" ] && continue
  emit "$where" "$message" "$fix"
done < "$FINDINGS_FILE"

exit "$drift"
