#!/usr/bin/env bash
# Test surface for .githooks/pre-commit — a commit stages only its own
# generated files (spec 017, scenario a-commit-stages-only-its-own-generated-files).
#
# The hook is this repository's own (the shipped adopter hook is a separate
# file). Each case builds a throwaway repository from this checkout's framework,
# generators and hook, commits a baseline through the hook, then makes a commit
# while a second session's unstaged sources and outputs sit in the working tree.
#
# Coverage:
#   A. the baseline commit through the hook succeeds and its outputs agree with
#      its sources (control: the later cases start from a healthy tree)
#   B. a hand edit to claude.md staged while another session's runtime-tools.txt
#      entry is unstaged: the commit's allow block and configure.md carry the
#      staged text only; the other session's three files stay byte-identical
#   C. a staged runtime-tools.txt entry reaches the committed outputs, regenerated,
#      and the clean working-tree outputs follow it; another session's unstaged
#      command source and output stay out of the commit and unchanged
#   D. a generated output staged by hand that disagrees with the sources is
#      committed regenerated
#   E. a commit staging no generator source stages no generated file, though the
#      committed outputs were already stale
#   F. a markdown file staged in part: the commit carries the normalized staged
#      text only, and the working copy keeps its unstaged hunks
#   G. a staged spec gets its derived frontmatter and labelled criteria; an
#      unstaged spec's derivation stays out of the commit and the working tree
#   H. a staged violation --fix cannot repair (MD028, a blank line inside a
#      blockquote) stops the commit, naming it, and leaves HEAD and the index
#      as they were (spec 017 task 46)
#
# HOOK_UNDER_TEST overrides the hook (scripts/mutation-style proofs that a case
# fails with a behavior removed). The runtime is ./runtime/target/release/ductus
# or `ductus` on PATH; without either the test fails rather than skipping.
#
# Usage: scripts/tests/test-pre-commit-staging.sh

# The `bash -c '...' _ args` cases expand their variables in the child shell.
# shellcheck disable=SC2016
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
HOOK="${HOOK_UNDER_TEST:-$REPO_ROOT/.githooks/pre-commit}"

# A run inside a hook or a worktree exports these; they would aim every git call
# below at the wrong repository.
unset GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE GIT_PREFIX GIT_COMMON_DIR \
  GIT_OBJECT_DIRECTORY GIT_ALTERNATE_OBJECT_DIRECTORIES

if [ -x "$REPO_ROOT/runtime/target/release/ductus" ]; then
  RUNTIME="$REPO_ROOT/runtime/target/release/ductus"
elif command -v ductus > /dev/null; then
  RUNTIME="$(command -v ductus)"
else
  echo "no ductus runtime: build runtime/target/release/ductus first" >&2
  exit 1
fi

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }
check() { # check <description> <command...>
  local desc="$1"; shift
  if "$@"; then pass "$desc"; else fail "$desc"; fi
}

WORK="$(mktemp -d)"
trap '[ -n "${KEEP_WORK:-}" ] || rm -rf "$WORK"' EXIT
[ -z "${KEEP_WORK:-}" ] || echo "keeping $WORK"

g() { local d="$1"; shift; git -C "$d" "$@"; }
head_has() { g "$1" show "HEAD:$2" | grep -qF -- "$3"; }
blob_of() { g "$1" hash-object -- "$2"; }

# A commit through this repository's hook, output kept for a failing case.
commit() { # commit <dir> <message> [git commit args...]
  local d="$1" m="$2"; shift 2
  g "$d" -c core.hooksPath=.githooks -c user.name=t -c user.email=t@example.com \
    commit -q -m "$m" "$@" > "$d/../commit.out" 2>&1
}

# The baseline: this checkout's framework and generators, three specs, a
# markdown file with room for two hunks, and a markdown normalizer that strips
# trailing whitespace (the real one is an npm install CI has and a fixture
# should not fetch).
BASE="$WORK/baseline"
build_baseline() {
  mkdir -p "$BASE/scripts" "$BASE/.githooks" "$BASE/.ductus" \
    "$BASE/runtime/target/release" "$BASE/node_modules/.bin"
  cp -R "$REPO_ROOT/framework" "$BASE/framework"
  cp "$REPO_ROOT"/scripts/gen-*.sh "$REPO_ROOT/scripts/lint-governing-size.sh" "$REPO_ROOT/scripts/maintainer-only-commands.txt" "$BASE/scripts/"
  cp "$HOOK" "$BASE/.githooks/pre-commit"
  cp "$REPO_ROOT/.markdownlint-cli2.jsonc" "$REPO_ROOT/.gitignore" "$REPO_ROOT/AGENTS.md" "$BASE/"
  cp "$REPO_ROOT/.ductus/config.toml" "$BASE/.ductus/"
  ln -s "$RUNTIME" "$BASE/runtime/target/release/ductus"
  cat > "$BASE/node_modules/.bin/markdownlint-cli2" <<'STUB'
#!/usr/bin/env bash
# Fixes trailing whitespace, and reports a blank line inside a blockquote
# (MD028, which --fix cannot repair) with exit 1, as the real tool does.
rc=0
for f in "$@"; do
  case "$f" in -*) continue ;; esac
  sed 's/[[:space:]]*$//' "$f" > "$f.tmp" && mv "$f.tmp" "$f"
  out="$(awk '/^>/ && p1 == "" && p2 ~ /^>/ { print FILENAME ":" NR " MD028/no-blanks-blockquote" } { p2 = p1; p1 = $0 }' "$f")"
  if [ -n "$out" ]; then printf '%s\n' "$out"; rc=1; fi
done
exit "$rc"
STUB
  chmod +x "$BASE/node_modules/.bin/markdownlint-cli2"
  printf 'node_modules/\nruntime/target/\n' >> "$BASE/.gitignore"
  local n
  for n in 001-alpha 002-beta 003-gamma; do
    mkdir -p "$BASE/specs/$n"
    printf -- '---\nstatus: draft\ndependencies: []\n---\n\n# %s\n\n## Acceptance Criteria\n\n- [ ] AC1: it works\n' \
      "$n" > "$BASE/specs/$n/spec.md"
  done
  {
    printf '# Readme\n\n'
    local i
    for i in $(seq 1 12); do printf 'top line %s\n' "$i"; done
    printf '\n'
    for i in $(seq 1 12); do printf 'bottom line %s\n' "$i"; done
  } > "$BASE/README.md"
  g "$BASE" init -q
  g "$BASE" add -- framework scripts .githooks .ductus specs README.md AGENTS.md \
    .gitignore .markdownlint-cli2.jsonc
}

# One fresh copy of the committed baseline per case.
new_case() {
  CASE="$WORK/$1"
  mkdir -p "$CASE"
  cp -R "$WORK/committed" "$CASE/repo"
  R="$CASE/repo"
}

# True when every generated output at HEAD agrees with the sources at HEAD.
outputs_in_sync() {
  local c="$CASE/sync"
  rm -rf "$c"
  git clone -q --no-hardlinks "$R" "$c"
  ( cd "$c" \
    && scripts/gen-configure-mcp.sh --dry-run > /dev/null 2>&1 \
    && DUCTUS_BIN="$RUNTIME" scripts/gen-claude-commands.sh --check > /dev/null 2>&1 \
    && scripts/gen-help-tables.sh --dry-run > /dev/null 2>&1 )
}

# What another session leaves behind: an unstaged edit to a generator source and
# the working-tree outputs regenerated from it.
other_session_regenerates() { # other_session_regenerates <dir>
  ( cd "$1" \
    && scripts/gen-configure-mcp.sh > /dev/null \
    && DUCTUS_BIN="$RUNTIME" scripts/gen-claude-commands.sh > /dev/null \
    && scripts/gen-help-tables.sh > /dev/null )
}

committed_files() { g "$1" diff-tree --no-commit-id --name-only -r HEAD; }

echo "Running pre-commit staging tests (hook: $HOOK)..."

build_baseline
# Generated outputs do not exist yet; the baseline commit goes through the hook
# so it makes them, which is also the first thing under test.
if commit "$BASE" baseline; then
  pass "A: the baseline commit goes through the hook"
else
  fail "A: the baseline commit failed in the hook: $(cat "$WORK/commit.out")"
  exit 1
fi
cp -R "$BASE" "$WORK/committed"
CASE="$WORK/a"; mkdir -p "$CASE"; R="$WORK/committed"
check "A: the baseline's outputs agree with its sources" outputs_in_sync
check "A: the baseline commit carries generated outputs" \
  g "$R" cat-file -e "HEAD:.claude/commands/ductus/configure.md"

# B. hand edit to claude.md staged; another session's runtime-tools.txt entry
# and regenerated outputs unstaged.
new_case b
CLAUDE=framework/bootstrap/configure/claude.md
CONFIGURE=.claude/commands/ductus/configure.md
printf '\n<!-- hand edit marker -->\n' >> "$R/$CLAUDE"
g "$R" add -- "$CLAUDE"
printf 'other-session-tool\n' >> "$R/framework/runtime-tools.txt"
other_session_regenerates "$R"
snap_tools="$(blob_of "$R" framework/runtime-tools.txt)"
snap_claude="$(blob_of "$R" "$CLAUDE")"
snap_conf="$(blob_of "$R" "$CONFIGURE")"
if commit "$R" "case b"; then
  pass "B: the commit succeeds"
else
  fail "B: the commit failed: $(cat "$CASE/commit.out")"
fi
check "B: the committed claude.md carries the staged hand edit" \
  head_has "$R" "$CLAUDE" "hand edit marker"
check "B: the committed allow block lacks the other session's tool" \
  bash -c '! git -C "$1" show "HEAD:$2" | grep -qF other-session-tool' _ "$R" "$CLAUDE"
check "B: configure.md is regenerated from the staged source" \
  head_has "$R" "$CONFIGURE" "hand edit marker"
check "B: configure.md lacks the other session's tool" \
  bash -c '! git -C "$1" show "HEAD:$2" | grep -qF other-session-tool' _ "$R" "$CONFIGURE"
check "B: runtime-tools.txt is not committed" \
  bash -c '! git -C "$1" show HEAD:framework/runtime-tools.txt | grep -qF other-session-tool' _ "$R"
check "B: the other session's runtime-tools.txt is byte-identical" \
  test "$(blob_of "$R" framework/runtime-tools.txt)" = "$snap_tools"
check "B: the other session's claude.md is byte-identical" \
  test "$(blob_of "$R" "$CLAUDE")" = "$snap_claude"
check "B: the other session's configure.md is byte-identical" \
  test "$(blob_of "$R" "$CONFIGURE")" = "$snap_conf"
check "B: the committed outputs agree with the committed sources" outputs_in_sync

# C. a staged source entry reaches the committed outputs; a second session's
# command source and output stay out.
new_case c
printf 'staged-tool\n' >> "$R/framework/runtime-tools.txt"
g "$R" add -- framework/runtime-tools.txt
printf '\nother session line\n' >> "$R/framework/commands/status.md"
STATUS_OUT=.claude/commands/ductus/status.md
( cd "$R" && DUCTUS_BIN="$RUNTIME" scripts/gen-claude-commands.sh > /dev/null )
snap_status_src="$(blob_of "$R" framework/commands/status.md)"
snap_status_out="$(blob_of "$R" "$STATUS_OUT")"
if commit "$R" "case c"; then
  pass "C: the commit succeeds"
else
  fail "C: the commit failed: $(cat "$CASE/commit.out")"
fi
check "C: the committed claude.md carries the staged tool" \
  head_has "$R" "$CLAUDE" "staged-tool"
check "C: the committed configure.md carries the staged tool" \
  head_has "$R" "$CONFIGURE" "staged-tool"
check "C: the committed outputs agree with the committed sources" outputs_in_sync
check "C: the clean working-tree claude.md follows the commit" \
  test -z "$(g "$R" status --porcelain -- "$CLAUDE" "$CONFIGURE")"
check "C: the other session's command source is not committed" \
  bash -c '! git -C "$1" show HEAD:framework/commands/status.md | grep -qF "other session line"' _ "$R"
check "C: the other session's command output is not committed" \
  bash -c '! git -C "$1" show "HEAD:$2" | grep -qF "other session line"' _ "$R" "$STATUS_OUT"
check "C: the other session's command source is byte-identical" \
  test "$(blob_of "$R" framework/commands/status.md)" = "$snap_status_src"
check "C: the other session's command output is byte-identical" \
  test "$(blob_of "$R" "$STATUS_OUT")" = "$snap_status_out"

# D. a hand-staged output that disagrees with the sources.
new_case d
printf '\nCORRUPT OUTPUT\n' >> "$R/$STATUS_OUT"
printf 'one more line\n' >> "$R/README.md"
g "$R" add -- "$STATUS_OUT" README.md
if commit "$R" "case d"; then
  pass "D: the commit succeeds"
else
  fail "D: the commit failed: $(cat "$CASE/commit.out")"
fi
check "D: the committed output is regenerated, not the staged corruption" \
  bash -c '! git -C "$1" show "HEAD:$2" | grep -qF "CORRUPT OUTPUT"' _ "$R" "$STATUS_OUT"
check "D: the committed outputs agree with the committed sources" outputs_in_sync

# E. a commit staging no generator source stages no generated file, though the
# committed outputs are already stale.
new_case e
printf 'stale-tool\n' >> "$R/framework/runtime-tools.txt"
g "$R" add -- framework/runtime-tools.txt
g "$R" -c user.name=t -c user.email=t@example.com commit -q --no-verify -m "stale outputs" \
  > /dev/null
printf 'a readme line\n' >> "$R/README.md"
g "$R" add -- README.md
if commit "$R" "case e"; then
  pass "E: the commit succeeds"
else
  fail "E: the commit failed: $(cat "$CASE/commit.out")"
fi
check "E: the commit holds README.md and nothing else" \
  test "$(committed_files "$R")" = "README.md"

# F. a markdown file staged in part.
new_case f
cp "$R/README.md" "$CASE/readme.orig"
sed 's/^top line 2$/top line 2 edited   /' "$CASE/readme.orig" > "$CASE/readme.staged"
sed 's/^bottom line 5$/bottom line 5 from another session/' "$CASE/readme.staged" > "$R/README.md"
blob="$(g "$R" hash-object -w "$CASE/readme.staged")"
g "$R" update-index --cacheinfo "100644,$blob,README.md"
snap_readme="$(blob_of "$R" README.md)"
if commit "$R" "case f"; then
  pass "F: the commit succeeds"
else
  fail "F: the commit failed: $(cat "$CASE/commit.out")"
fi
check "F: the committed text is the staged hunk, normalized" \
  test "$(g "$R" show HEAD:README.md)" = "$(sed 's/^top line 2$/top line 2 edited/' "$CASE/readme.orig")"
check "F: the other session's hunk is not committed" \
  bash -c '! git -C "$1" show HEAD:README.md | grep -qF "from another session"' _ "$R"
check "F: the working copy is byte-identical, unstaged hunk and all" \
  test "$(blob_of "$R" README.md)" = "$snap_readme"
check "F: the unstaged hunk is still pending" \
  bash -c 'git -C "$1" diff --quiet -- README.md; test $? -eq 1' _ "$R"

# G. a staged spec is derived and labelled; an unstaged one is left alone.
new_case g
printf '\nSee [alpha](../001-alpha/spec.md).\n- [ ] a new criterion\n' >> "$R/specs/002-beta/spec.md"
g "$R" add -- specs/002-beta/spec.md
printf '\nSee [alpha](../001-alpha/spec.md).\n' >> "$R/specs/003-gamma/spec.md"
snap_gamma="$(blob_of "$R" specs/003-gamma/spec.md)"
if commit "$R" "case g"; then
  pass "G: the commit succeeds"
else
  fail "G: the commit failed: $(cat "$CASE/commit.out")"
fi
check "G: the staged spec's dependencies are derived" \
  bash -c 'git -C "$1" show HEAD:specs/002-beta/spec.md | grep -qF 001-alpha' _ "$R"
check "G: the staged spec's new criterion is labelled" \
  bash -c 'git -C "$1" show HEAD:specs/002-beta/spec.md | grep -qE "^- \[ \] AC2: a new criterion"' _ "$R"
check "G: the working copy of the staged spec is the committed one" \
  test -z "$(g "$R" status --porcelain -- specs/002-beta/spec.md)"
check "G: the unstaged spec is not committed" \
  test "$(g "$R" show HEAD:specs/003-gamma/spec.md | grep -c 'dependencies: \[\]')" = 1
check "G: the unstaged spec is byte-identical" \
  test "$(blob_of "$R" specs/003-gamma/spec.md)" = "$snap_gamma"

# H. a violation --fix cannot repair stops the commit.
new_case h
head_before="$(g "$R" rev-parse HEAD)"
printf '\n> one quote\n\n> another quote\n' >> "$R/README.md"
g "$R" add -- README.md
index_before="$(g "$R" write-tree)"
if commit "$R" "case h"; then
  fail "H: the commit landed with an MD028 the hook should have refused"
else
  pass "H: the commit is refused"
fi
check "H: the refusal names the violation" \
  grep -qF "MD028/no-blanks-blockquote" "$CASE/commit.out"
check "H: HEAD did not move" \
  test "$(g "$R" rev-parse HEAD)" = "$head_before"
check "H: the index is as it was staged" \
  test "$(g "$R" write-tree)" = "$index_before"

if [ "$failures" -ne 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
echo "All pre-commit staging tests passed."
