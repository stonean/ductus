#!/usr/bin/env bash
# scripts/audit/self-url-resolution.sh — Family 36 of /audit.
#
# An absolute GitHub URL that points back into THIS repository and names a path
# that no longer exists.
#
# The form exists because of Family 35's finding. A file the manifest ships
# cannot cite ductus's own spec corpus with a relative link: `specs/` is this
# project's development record and is not in the manifest, so `../../specs/017-…`
# is a claim about a neighbour the adopter does not have. The repair was to
# name the repository instead of assuming it —
# `https://github.com/stonean/ductus/blob/main/specs/017-…` — which resolves
# from a maintainer's checkout and an adopter's alike, because it no longer
# depends on where the citing file sits.
#
# That trade bought correctness in the adopter tree and spent verifiability
# here. A relative link is checked by Family 26 and by `check-corpus-links` on
# every commit; an absolute one is checked by nobody, so renaming or
# consolidating a spec leaves it pointing at a 404 that only a reader clicking
# it discovers — and readers of a shipped rule file are adopters, who cannot
# tell a ductus defect from their own. Silent rot in the reassuring direction,
# which is the shape `QUAL-CLAIM-001` names and this suite exists to refuse.
#
# THE CHECK IS THE OBVIOUS ONE, AND THAT IS THE POINT. Every such URL has the
# form `<repo>/blob/<ref>/<path>`, where `<path>` is a path in this repository.
# Strip the prefix and test the remainder against the worktree. No network, no
# rate limit, no flake: the check is stronger than the relative-link check it
# replaced, because there is no depth arithmetic to get wrong — the reason the
# dominant Family 26 finding is a scenario file one `../` short.
#
# THE REPOSITORY IS DERIVED, NOT HARDCODED. The slug comes from the installer's
# own archive URL in `framework/bootstrap/ductus.md`, the single place ductus
# states its canonical repository. Hardcoding `stonean/ductus` here would be a
# second copy of exactly the fact under test, and it would make every finding
# wrong in a fork rather than merely absent.
#
# WHAT IS AND IS NOT A SUBJECT.
#
#   - `blob/main/<path>` and `tree/main/<path>` are resolved. `main` is the
#     trunk — this project is live-on-main, and everything `/ductus` fetches
#     tracks it — so a `main` URL is a claim about the current tree and the
#     worktree is the right authority for it.
#   - A URL pinned to a tag or a sha is deliberately historical: it names a
#     state the worktree is not. Those are COUNTED and reported on stderr, not
#     resolved and not silently dropped, because "we did not check these" and
#     "these were fine" must not read alike.
#   - A URL carrying a `{placeholder}`, or an ellipsis standing in for the rest
#     of a path, is a documented shape rather than a pointer and is excluded by
#     construction. Both markers are exact: the release download URLs in the
#     installer are the standing `{placeholder}` example, and prose explaining
#     this very URL form necessarily writes the ellipsis — this family's own
#     scenario did, and the family reported it on its first run. No real path
#     segment is `…` or `...`, so neither exclusion can hide a live link.
#   - Generated command copies under the host config dir are not a subject:
#     they carry whatever their source carries, and the repair is a ductus
#     release, not an edit there. Family 26 and Family 34 draw the same line.
#
# A `blob` URL naming a directory, and a `tree` URL naming a file, are reported
# separately from an unresolved path. GitHub redirects both, so neither is
# broken today and neither is a 404 waiting to happen — but the two repairs
# differ from each other and from a genuine miss, and a family that folds three
# repairs into one message sends a maintainer looking for the wrong thing.
#
# An empty markdown corpus is a finding. Zero URLs is NOT: a repository that
# cites nothing absolutely is legitimately clean, and the count on stderr is
# what keeps that from reading as an extraction that silently broke.
#
# Bash 3.2 compatible (macOS system bash). The extraction computes in python3
# and renders tab-separated records the shell feeds to `emit` — the convention
# ./README.md records for the families that compute in python.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family self-url

MANIFEST_FILE="framework/bootstrap/ductus.md"
SELF="scripts/audit/self-url-resolution.sh"

# The canonical repository, as the installer states it. `.../archive/refs/heads/main.tar.gz`
# is the archive every `/ductus` run fetches, so the slug in it is the one
# authority this repository has for its own identity.
repo_slug="$(grep -oE 'https://github\.com/[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+/archive/' "$MANIFEST_FILE" 2> /dev/null \
  | head -1 \
  | sed -E 's#https://github\.com/([A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+)/archive/#\1#')"

if [ -z "$repo_slug" ]; then
  emit "$MANIFEST_FILE" \
    "could not derive this repository's canonical slug from the installer's archive URL — no self-referencing URL was resolved" \
    "restore the https://github.com/<owner>/<repo>/archive/refs/heads/main.tar.gz URL in the File Fetching section, or update this family's extraction"
  exit "$drift"
fi

# The host config dir holds generated copies; excluded for the same reason
# Families 26 and 34 exclude it.
config_dir="$(sed -n -E 's/^[[:space:]]*cli-config-dir[[:space:]]*=[[:space:]]*"([^"]+)".*/\1/p' .ductus/session.toml 2> /dev/null | head -1)"
[ -n "$config_dir" ] || config_dir=".claude"

files="$(git ls-files '*.md' 2> /dev/null | grep -v "^${config_dir}/" | sort)"
file_count="$(printf '%s\n' "$files" | grep -c '[^[:space:]]')"

if [ "$file_count" -eq 0 ]; then
  emit "$SELF" \
    "no tracked markdown files were enumerated — the self-URL scan examined nothing" \
    "check that this runs inside the repository and that git ls-files resolves"
  exit "$drift"
fi

records="$(printf '%s\n' "$files" | REPO_SLUG="$repo_slug" python3 -c '
import os, re, sys
from urllib.parse import unquote

slug = re.escape(os.environ["REPO_SLUG"])
URL = re.compile(r"https://github\.com/" + slug + r"/(blob|tree)/([^/\s)]+)/([^)\s\"'"'"'>\]]+)")

examined = 0
urls = 0
pinned = 0
for path in (line.strip() for line in sys.stdin):
    if not path:
        continue
    try:
        with open(path, encoding="utf-8") as handle:
            lines = handle.read().splitlines()
    except OSError as err:
        print("unreadable\t%s\t\t%s" % (path, err.strerror))
        continue
    examined += 1
    for number, line in enumerate(lines, 1):
        for kind, ref, target in URL.findall(line):
            # A documented shape, not a pointer. Two markers, both exact:
            # a {placeholder}, and an ellipsis standing in for the rest of a
            # path. Prose explaining this URL form necessarily writes one, as
            # the scenario and registry entries for this family do, and no
            # real path segment is an ellipsis — so neither exclusion can
            # hide a live link. (No apostrophes in this block: the whole
            # program is a single-quoted shell string.)
            if any(mark in target for mark in ("{", "}", "…", "...")):
                continue
            urls += 1
            if ref != "main":
                pinned += 1
                continue
            # Trailing punctuation belongs to the prose, not the path; a
            # fragment (#L12, #anchor) and a query are not part of it either.
            clean = unquote(target.split("#", 1)[0].split("?", 1)[0]).rstrip(".,;:")
            if not clean:
                continue
            if not os.path.exists(clean):
                print("unresolved\t%s\t%d\t%s" % (path, number, clean))
            elif kind == "blob" and os.path.isdir(clean):
                print("blob-names-dir\t%s\t%d\t%s" % (path, number, clean))
            elif kind == "tree" and os.path.isfile(clean):
                print("tree-names-file\t%s\t%d\t%s" % (path, number, clean))
print("counts\t%d\t%d\t%d" % (examined, urls, pinned))
')"

examined=0
urls=0
pinned=0
while IFS=$'\t' read -r kind f1 f2 f3; do
  case "$kind" in
    unresolved)
      emit "$f1:$f2" \
        "absolute link into this repository names \`$f3\`, which does not exist — the URL is a 404 no local check would otherwise see" \
        "re-point it at the path the target moved to, or name the target in prose when it was removed"
      ;;
    blob-names-dir)
      emit "$f1:$f2" \
        "absolute link uses /blob/ for \`$f3\`, which is a directory" \
        "use /tree/ for a directory; GitHub redirects today, so nothing else will report this"
      ;;
    tree-names-file)
      emit "$f1:$f2" \
        "absolute link uses /tree/ for \`$f3\`, which is a file" \
        "use /blob/ for a file; GitHub redirects today, so nothing else will report this"
      ;;
    unreadable)
      emit "$f1" \
        "tracked markdown file could not be read ($f3) — its absolute links were not examined" \
        "resolve the read failure; an unexaminable file must not be counted as clean"
      ;;
    counts)
      examined="$f1"
      urls="$f2"
      pinned="$f3"
      ;;
  esac
done <<< "$records"

if [ "$examined" -eq 0 ]; then
  emit "$SELF" \
    "no tracked markdown file could be read — the self-URL scan examined nothing" \
    "resolve the read failures above; a scan of nothing must not exit like a clean one"
fi

# The counts are the guard. `urls` is the one that matters: it is the only
# thing separating "every self-referencing URL resolves" from "the extraction
# quietly stopped matching any", and the two exit identically.
echo "self-url: ${examined} tracked markdown file(s) examined against ${repo_slug}; ${urls} self-referencing URL(s) found, ${pinned} pinned to a non-main ref and therefore not resolvable against this worktree; generated copies under ${config_dir}/ excluded by construction" >&2

exit "$drift"
