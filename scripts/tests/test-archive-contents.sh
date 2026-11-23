#!/usr/bin/env bash
# Test surface for scripts/audit/archive-contents.sh (Family 42).
#
# The family holds the archive `/ductus` downloads, `git archive` of HEAD with
# the commit's own `.gitattributes`, to the files a run reads, derived from
# the installer's two halves (spec 015 AC14). The evidence it works is that it
# FAILS when the archive omits a file a run reads and when it carries one no
# run reads, and that a read set it could not derive never reads as a pass.
# Each fixture is a small repository holding the family, its lib, an installer
# naming its sources the ways the real one does, and files no run reads, so
# the real `.gitattributes` is never edited.
#
# Coverage:
#   A. the fixture passes, its coverage line naming 11 files read, archived
#      and in both; destinations it tracks (AGENTS.md, .gitignore) and the
#      withheld `audit` command count as no read
#   B. a read file marked export-ignore fails, naming it as omitted
#   C. an unread framework/ file left unmarked fails, naming it as carried
#   D. a top-level file left unmarked fails, naming it as carried
#   E. `{key}` expands over the Agent Registry: one key's configure file
#      marked export-ignore fails, naming it
#   F. `{id}` expands over the migrations registry: one migration file marked
#      export-ignore fails, naming it
#   G. a glob expands over the tracked tree: one rule file marked
#      export-ignore fails, naming it
#   H. a manifest source outside framework/ marked export-ignore fails
#   I. a missing maintainer-only list fails, and the withheld command is
#      then named as omitted
#   J. a framework/ path with a placeholder the family does not expand fails
#   K. an unreadable installer half fails rather than passes
#   L. a `{key}` path with no Agent Registry, and an `{id}` path with no
#      migration id, each fail
#   M. a directory with no commit fails rather than passes
#   N. the real tree passes at HEAD
#
# Usage: scripts/tests/test-archive-contents.sh

# The fixture installer's backticks are markdown code spans, written literally.
# shellcheck disable=SC2016
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
FAMILY="scripts/audit/archive-contents.sh"
LIB="scripts/audit/lib.sh"

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }
gitc() { git -c user.email=t@example.invalid -c user.name=t -c commit.gpgsign=false "$@"; }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# write DIR PATH CONTENT — write one fixture file.
write() {
  mkdir -p "$(dirname "$1/$2")"
  printf '%s\n' "$3" > "$1/$2"
}

# make_tree NAME [nocommit] — a fixture repository, committed unless asked.
make_tree() {
  local dir="$WORK/$1" f
  mkdir -p "$dir"
  for f in "$FAMILY" "$LIB"; do
    mkdir -p "$dir/$(dirname "$f")"
    cp "$REPO_ROOT/$f" "$dir/$f"
  done
  write "$dir" framework/bootstrap/ductus.md '# ductus

## Agent Registry

| `key` | `name` | `config_dir` |
| --- | --- | --- |
| `claude` | Claude | `.claude` |
| `pi` | Pi | `.pi` |

## Shared Files

| Source Path | Destination Path |
| --- | --- |
| `framework/constitution.md` | `.ductus/constitution.md` |
| `.markdownlint-cli2.jsonc` | `.markdownlint-cli2.jsonc` |
| `framework/bootstrap/configure/{key}.md` | `{config_dir}/commands/x/configure.md` |

Load every `framework/rules/*.md` file, then read
`{framework-root}/framework/bootstrap/ductus-procedure.md`. The maintainer-only
`framework/commands/audit.md` is withheld. A run writes `AGENTS.md` and
`.gitignore` in the adopter tree and lints `**/*.md` there.'
  write "$dir" framework/bootstrap/ductus-procedure.md 'Read `framework/migrations.toml`, then run each `framework/migrations/{id}.md`
it names.'
  write "$dir" framework/migrations.toml '[[migrations]]
id = "one"

[[migrations]]
id = "two"'
  write "$dir" framework/migrations/one.md 'one'
  write "$dir" framework/migrations/two.md 'two'
  write "$dir" framework/constitution.md 'constitution'
  write "$dir" framework/bootstrap/configure/claude.md 'claude'
  write "$dir" framework/bootstrap/configure/pi.md 'pi'
  write "$dir" framework/rules/a-cross.md 'a'
  write "$dir" framework/rules/b-backend.md 'b'
  write "$dir" framework/commands/audit.md 'audit'
  write "$dir" framework/reference/audit.md 'audit reference'
  write "$dir" framework/runtime-tools.txt 'tools'
  write "$dir" .markdownlint-cli2.jsonc '{}'
  write "$dir" AGENTS.md 'agents'
  write "$dir" .gitignore 'target/'
  write "$dir" specs/001-x/spec.md 'spec'
  write "$dir" scripts/maintainer-only-commands.txt '# withheld
audit'
  write "$dir" .gitattributes '* export-ignore
/framework -export-ignore
/framework/** -export-ignore
/.markdownlint-cli2.jsonc -export-ignore
/framework/runtime-tools.txt export-ignore
/framework/commands/audit.md export-ignore
/framework/reference/audit.md export-ignore'
  (cd "$dir" && git init -q)
  if [ "${2:-}" != nocommit ]; then
    (cd "$dir" && gitc add -A && gitc commit -q -m base)
  fi
  echo "$dir"
}

# commit DIR — commit the fixture's edits.
commit() { (cd "$1" && gitc add -A && gitc commit -q -m edit); }

# mark DIR LINE — append a line to the fixture's .gitattributes and commit.
mark() { printf '%s\n' "$2" >> "$1/.gitattributes"; commit "$1"; }

# unmark DIR LINE — remove an exact line from the fixture's .gitattributes
# and commit; a line that is not there is a broken test, not a pass.
unmark() {
  grep -vxF -- "$2" "$1/.gitattributes" > "$1/.gitattributes.new" || true
  if cmp -s "$1/.gitattributes" "$1/.gitattributes.new"; then
    echo "unmark: $2 is not in the fixture" >&2
    exit 1
  fi
  mv "$1/.gitattributes.new" "$1/.gitattributes"
  commit "$1"
}

# run_family DIR — run the fixture's copy; sets out, err and code.
run_family() {
  set +e
  out="$("$BASH" "$1/$FAMILY" 2> "$WORK/stderr")"
  code=$?
  set -e
  err="$(cat "$WORK/stderr")"
}

# expect NAME CODE [SUBSTRING...] — check the last run.
expect() {
  local name="$1" want="$2" ok=1 needle
  shift 2
  [ "$code" -eq "$want" ] || ok=0
  for needle in "$@"; do
    case "$out$err" in *"$needle"*) ;; *) ok=0 ;; esac
  done
  if [ "$ok" -eq 1 ]; then pass "$name"; else
    fail "$name (exit $code)"
    printf '%s\n%s\n' "$out" "$err" >&2
  fi
}

OMITS="the archive omits it"
CARRIES="no /ductus run reads it"

# A. the fixture
t="$(make_tree a)"; run_family "$t"
expect "A. the fixture passes, 11 read, archived and in both" 0 \
  "11 file(s) in the read set, 11 in the archive, 11 in both"

# B. a read file marked
t="$(make_tree b)"; mark "$t" "/framework/constitution.md export-ignore"; run_family "$t"
expect "B. an omitted read file fails" 1 "framework/constitution.md | a /ductus run reads this file, but $OMITS"

# C. an unread framework/ file unmarked
t="$(make_tree c)"; unmark "$t" "/framework/runtime-tools.txt export-ignore"; run_family "$t"
expect "C. an unread framework/ file carried fails" 1 "framework/runtime-tools.txt | the archive carries this file, but $CARRIES"

# D. a top-level file unmarked
t="$(make_tree d)"; unmark "$t" "* export-ignore"; run_family "$t"
expect "D. a top-level file carried fails" 1 "specs/001-x/spec.md | the archive carries" "AGENTS.md | the archive carries"

# E. {key}
t="$(make_tree e)"; mark "$t" "/framework/bootstrap/configure/pi.md export-ignore"; run_family "$t"
expect "E. a {key} source omitted fails" 1 "framework/bootstrap/configure/pi.md | a /ductus run reads"

# F. {id}
t="$(make_tree f)"; mark "$t" "/framework/migrations/two.md export-ignore"; run_family "$t"
expect "F. an {id} source omitted fails" 1 "framework/migrations/two.md | a /ductus run reads"

# G. a glob
t="$(make_tree g)"; mark "$t" "/framework/rules/b-backend.md export-ignore"; run_family "$t"
expect "G. a globbed source omitted fails" 1 "framework/rules/b-backend.md | a /ductus run reads"

# H. a manifest source outside framework/
t="$(make_tree h)"; unmark "$t" "/.markdownlint-cli2.jsonc -export-ignore"; run_family "$t"
expect "H. the root manifest source omitted fails" 1 ".markdownlint-cli2.jsonc | a /ductus run reads"

# I. no maintainer-only list
t="$(make_tree i)"; rm "$t/scripts/maintainer-only-commands.txt"; commit "$t"; run_family "$t"
expect "I. a missing maintainer-only list fails" 1 \
  "scripts/maintainer-only-commands.txt | cannot be read at HEAD" "framework/commands/audit.md | a /ductus run reads"

# J. an unexpanded placeholder
t="$(make_tree j)"
printf '%s\n' 'Copy `framework/templates/{lang}.md` too.' >> "$t/framework/bootstrap/ductus.md"; commit "$t"
run_family "$t"
expect "J. an unexpanded placeholder fails" 1 "whose placeholder \`{lang}\` this family does not expand" "1 unexpanded placeholder"

# K. an unreadable half
t="$(make_tree k)"; rm "$t/framework/bootstrap/ductus-procedure.md"; commit "$t"; run_family "$t"
expect "K. a missing installer half fails" 1 "framework/bootstrap/ductus-procedure.md | cannot be read at HEAD"

# L. nothing to expand a placeholder with
t="$(make_tree l1)"
python3 -I -c 'import sys
p = sys.argv[1]
text = open(p).read().replace("`key` | `name` | `config_dir`", "`id` | `label` | `dir`")
open(p, "w").write(text)' "$t/framework/bootstrap/ductus.md"
commit "$t"; run_family "$t"
expect "L. a {key} path with no Agent Registry fails" 1 "no Agent Registry key was read"
t="$(make_tree l2)"; printf '# empty\n' > "$t/framework/migrations.toml"; commit "$t"; run_family "$t"
expect "L. an {id} path with no migration id fails" 1 "no migration id was read"

# M. no commit
t="$(make_tree m nocommit)"; run_family "$t"
expect "M. a directory with no commit fails" 1 "could not be listed"

# N. the real tree, at HEAD
set +e
real_out="$("$BASH" "$REPO_ROOT/$FAMILY" 2> "$WORK/stderr")"
real_code=$?
set -e
if [ "$real_code" -eq 0 ] && grep -q "in the read set" "$WORK/stderr"; then
  pass "N. the real tree passes at HEAD ($(cat "$WORK/stderr"))"
else
  fail "N. the real tree (exit $real_code): $(printf '%s' "$real_out" | head -5)"
fi

if [ "$failures" -ne 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
echo "all cases passed"
