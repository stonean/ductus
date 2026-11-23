#!/usr/bin/env bash
# scripts/audit/archive-contents.sh — Family 42 of /audit.
#
# The archive `/ductus` downloads carries exactly the files a run reads from
# it, and no other tracked file (spec 015 AC14).
#
# A run fetches GitHub's source tarball of this repository at the ref it
# resolved, from codeload.github.com, and GitHub builds that tarball with
# `git archive`, which leaves out each path the archived commit's own
# `.gitattributes` marks `export-ignore`. Before 015's task 7 nothing was
# marked, so every run downloaded the whole repository, 1,424 files and
# 6,096,062 bytes gzipped at ductus-v0.64.0, to read 78 of them. The allow-list
# in `.gitattributes` now trims it; this family is what holds the trim to the
# installer, in both directions:
#
#   - a file a run reads that the archive omits is a finding, since the run
#     then warns `Source not found in archive` and installs nothing for it;
#   - a file the archive carries that no run reads is a finding, since every
#     adopter downloads it on every run for nothing.
#
# THE READ SET IS DERIVED FROM THE INSTALLER, NEVER LISTED (015 plan, The
# archive carries only what a run reads). It is every tracked file under
# `framework/` that `framework/bootstrap/ductus.md` or
# `framework/bootstrap/ductus-procedure.md` names in a code span, and the
# source cell of each manifest row (a table row whose first two cells are each
# one code span), with `{key}` expanded over the Agent Registry's keys, `{id}`
# over `framework/migrations.toml`'s ids, and a glob over the tracked tree,
# plus the two halves themselves, minus each command
# `scripts/maintainer-only-commands.txt` names and that command's reference.
# Only sources count: a destination the halves name is no read from the
# archive, though this repository tracks many of them. Naming stands in for
# reading: a run reads only what its instructions name, so the set omits
# nothing a run reads, while a file named for another reason counts as read,
# which the maintainer-only subtraction covers for the one such pair. A
# `framework/` path holding a placeholder this family does not expand is a
# finding, since the files it stands for were not derived. A manifest source
# that does not exist is Family 35's finding, not this one's.
#
# EVERYTHING IS READ AT HEAD — the tracked tree, the two halves, the
# registry, the maintainer-only list and the archive — because the archive
# GitHub serves is a commit's, attributes included. Uncommitted edits are
# invisible here; run the family after committing, as run-all.sh is run.
# `git archive` reads the tree, not history, so a shallow checkout suffices.
#
# A run that could not derive the read set or list the archive is a finding,
# never a pass, and every run ends with a coverage line on stderr naming both
# counts. scripts/tests/test-archive-contents.sh proves each direction fails.
#
# Bash 3.2 compatible (macOS system bash); the python runs on 3.9.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family archive-contents

INSTALLER="framework/bootstrap/ductus.md"
PROCEDURE="framework/bootstrap/ductus-procedure.md"
REGISTRY="framework/migrations.toml"
MAINTAINER_ONLY="scripts/maintainer-only-commands.txt"

if ! command -v python3 > /dev/null 2>&1; then
  emit "$0" "python3 is missing, so the read set was not derived and the archive was not compared" "install python3, then run the family again"
  exit "$drift"
fi

# derive — print one tab-separated line per result: `finding LOCATION MESSAGE
# FIX`, then exactly one `coverage LINE`. A function, not an inline
# `"$(python3 - <<...)"`, because bash 3.2 misparses a heredoc holding
# backticks inside a command substitution.
derive() {
  python3 -I - "$INSTALLER" "$PROCEDURE" "$REGISTRY" "$MAINTAINER_ONLY" << 'PY'
import fnmatch
import io
import re
import subprocess
import sys
import tarfile

INSTALLER, PROCEDURE, REGISTRY, MAINTAINER_ONLY = sys.argv[1:5]


def say(*fields):
    print("\t".join(f.replace("\t", " ").replace("\n", " ") for f in fields))


def finding(where, message, fix):
    say("finding", where, message, fix)


def git(*args):
    return subprocess.run(["git"] + list(args), stdout=subprocess.PIPE,
                          stderr=subprocess.PIPE, check=True).stdout


def at_head(path):
    try:
        return git("show", "HEAD:" + path).decode("utf-8")
    except (subprocess.CalledProcessError, UnicodeDecodeError):
        return None


try:
    tracked = set(p.decode("utf-8") for p in
                  git("ls-tree", "-r", "-z", "--name-only", "HEAD").split(b"\0") if p)
except (OSError, subprocess.CalledProcessError, UnicodeDecodeError):
    finding("HEAD", "the tracked tree could not be listed, so no read set was derived",
            "run the family inside a git work tree with at least one commit")
    say("coverage", "archive-contents: compared nothing (no tracked tree at HEAD)")
    sys.exit(0)

halves = {}
for path in (INSTALLER, PROCEDURE):
    text = at_head(path)
    if text is None:
        finding(path, "cannot be read at HEAD, so the read set was not derived",
                "restore the file, or update this family if the installer moved")
    else:
        halves[path] = text

names_text = at_head(MAINTAINER_ONLY)
withheld = set()
if names_text is None:
    finding(MAINTAINER_ONLY, "cannot be read at HEAD, so the withheld commands were not subtracted",
            "restore the file; an empty set would count a withheld command as read")
else:
    for line in names_text.splitlines():
        name = re.sub(r"\s", "", line.split("#", 1)[0])
        if name:
            withheld.add("framework/commands/%s.md" % name)
            withheld.add("framework/reference/%s.md" % name)
    if not withheld:
        finding(MAINTAINER_ONLY, "names no command, so nothing was subtracted",
                "list each maintainer-only command, as lib.sh maintainer_only_commands reads it")

keys = []
if INSTALLER in halves:
    in_table = False
    for line in halves[INSTALLER].splitlines():
        if re.match(r"^\| *`key` *\| *`name` *\| *`config_dir`", line):
            in_table = True
            continue
        if in_table:
            if re.match(r"^\| *-", line):
                continue
            if not line.startswith("|"):
                break
            key = line.split("|")[1].strip().strip("`").strip()
            if key:
                keys.append(key)

registry_text = at_head(REGISTRY)
ids = []
if registry_text is not None:
    ids = re.findall(r'(?m)^\s*id\s*=\s*"([^"]+)"', registry_text)

PLACEHOLDER = re.compile(r"\{[^}]*\}")
read = set(halves)
unexpanded = 0


def resolve(token, where):
    global unexpanded
    if "{key}" in token:
        if not keys:
            finding(where, "names `%s`, but no Agent Registry key was read to expand it" % token,
                    "restore the Agent Registry table in %s" % INSTALLER)
            return
        candidates = [token.replace("{key}", k) for k in keys]
    else:
        candidates = [token]
    if "{id}" in token:
        if not ids:
            finding(where, "names `%s`, but no migration id was read from %s to expand it" % (token, REGISTRY),
                    "restore the registry, whose entries each carry an id")
            return
        candidates = [c.replace("{id}", i) for c in candidates for i in ids]
    for path in candidates:
        left = PLACEHOLDER.search(path)
        if left:
            if path.startswith("framework/"):
                unexpanded += 1
                finding(where, "names `%s`, whose placeholder `%s` this family does not expand, so the files it stands for were not derived" % (token, left.group(0)),
                        "expand the placeholder in scripts/audit/archive-contents.sh, or spell the paths out")
            continue
        if "*" in path or "?" in path or "[" in path:
            read.update(t for t in tracked if fnmatch.fnmatchcase(t, path))
        elif path in tracked:
            read.add(path)


SPAN = re.compile(r"`([^`\n]+)`")
FRAMEWORK_PATH = re.compile(r"(?:^|(?<=[\s(\"=]))(?:\{framework-root\}/)?(framework/[\w./{}*?-]+)")
ROW = re.compile(r"^\| *`([^`]+)` *\| *`([^`]+)` *\|")
for path, text in sorted(halves.items()):
    for number, line in enumerate(text.splitlines(), 1):
        where = "%s:%d" % (path, number)
        for span in SPAN.finditer(line):
            for token in FRAMEWORK_PATH.finditer(span.group(1)):
                resolve(token.group(1).rstrip("."), where)
        row = ROW.match(line)
        if row:
            resolve(row.group(1), where)

read -= withheld

try:
    archive = git("archive", "--format=tar", "HEAD")
    members = tarfile.open(fileobj=io.BytesIO(archive), mode="r:").getmembers()
except (OSError, subprocess.CalledProcessError, tarfile.TarError):
    finding("HEAD", "git archive of HEAD could not be produced or read, so the archive was not compared",
            "run the family inside a git work tree with at least one commit")
    say("coverage", "archive-contents: read set of %d file(s) derived; archive not listed" % len(read))
    sys.exit(0)
archived = set(m.name for m in members if not m.isdir())

for path in sorted(read - archived):
    finding(path, "a /ductus run reads this file, but the archive omits it, so the run installs nothing for it",
            "stop marking it export-ignore in .gitattributes")
for path in sorted(archived - read):
    finding(path, "the archive carries this file, but no /ductus run reads it, so every run downloads it for nothing",
            "mark it export-ignore in .gitattributes, or name it in the installer if a run should read it")

if not read and halves:
    finding(INSTALLER, "the installer names no file under framework/, so the read set is empty",
            "the code spans this family reads no longer match; update the family with the installer")

say("coverage", "archive-contents: %d file(s) in the read set, %d in the archive, %d in both (%d unexpanded placeholder(s))"
    % (len(read), len(archived), len(read & archived), unexpanded))
PY
}

result="$(derive)"
status=$?
coverage=0
while IFS=$'\t' read -r kind location message fix; do
  case "$kind" in
    finding) emit "$location" "$message" "$fix" ;;
    coverage)
      echo "$location" >&2
      coverage=$((coverage + 1))
      ;;
  esac
done <<< "$result"

if [ "$status" -ne 0 ] || [ "$coverage" -ne 1 ]; then
  emit "$0" "the derivation exited $status with $coverage coverage line(s), so the comparison did not finish" \
    "run the family directly and read the python error"
fi

exit "$drift"
