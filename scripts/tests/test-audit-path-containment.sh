#!/usr/bin/env bash
# Test surface for the path containment of /audit Families 35 and 36
# (scripts/audit/manifest-destination-links.sh, scripts/audit/self-url-resolution.sh;
# spec 026 task 5, BE-INPUT-004).
#
# Each family turns text committed to the repository into a path: Family 35 a
# manifest source it copies (35a) or reads (35b), Family 36 a URL's path it
# probes and a tracked markdown file it reads. Each canonicalizes the path and
# compares it with the repository root by whole path components before
# touching it, so a symbolic link in the repository resolving outside it is
# reported and never copied, probed or read. Step (h) runs both families over
# this repository, which holds no such link, so a family that stopped
# containing would pass there.
#
# Coverage:
#   A. control: regular files and a symbolic link to a file inside the tree
#      pass both families
#   B. Family 36: a URL naming a symbolic link that resolves outside is reported
#      and not probed; the target sits in a sibling directory whose name the
#      root's is a prefix of, so a string-prefix check would accept it
#   C. Family 36: a tracked markdown file that is a symbolic link resolving
#      outside is reported and not read
#   D. Family 35a: a Shared Files source resolving outside is reported and not
#      copied
#   E. Family 35b: a slash-command source resolving outside is reported and not
#      read
#
# Needs the release runtime (runtime/target/release/ductus), which 35a runs
# check-corpus-links from, as framework-checks.yml builds it before this step.
#
# Usage: scripts/tests/test-audit-path-containment.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
BIN="$REPO_ROOT/runtime/target/release/ductus"

if [ ! -x "$BIN" ]; then
  echo "no release runtime at $BIN — build it (cargo build --release in runtime/) before this test; Family 35a cannot run without it" >&2
  exit 1
fi

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# make_tree CASE — a fixture repository holding both families, their lib, the
# runtime as an adopter-owned pointer, a manifest naming one Shared Files row
# and one slash-command row, the files those rows ship, a markdown file citing
# one self-URL, and, beside the root, a directory `repo2` holding a file
# outside it. Prints the directory.
make_tree() {
  local dir="$WORK/$1/repo"
  mkdir -p "$dir/scripts/audit" "$dir/framework/bootstrap" "$dir/framework/rules" \
    "$dir/framework/commands" "$dir/docs" "$dir/.ductus/bin" "$WORK/$1/repo2"
  cp "$REPO_ROOT/scripts/audit/lib.sh" "$REPO_ROOT/scripts/audit/manifest-destination-links.sh" \
    "$REPO_ROOT/scripts/audit/self-url-resolution.sh" "$dir/scripts/audit/"
  ln -s "$BIN" "$dir/.ductus/bin/ductus"
  cat > "$dir/framework/bootstrap/ductus.md" << 'EOF'
# ductus

Fetch https://codeload.github.com/acme/widget/tar.gz/{archive-ref} and extract it.

## Shared Files

| Source | Destination | Strategy |
| --- | --- | --- |
| `framework/rules/security-cross.md` | `specs/rules/security-cross.md` | update |

### Slash commands (strategy: update)

| Source | Destination |
| --- | --- |
| `framework/commands/target.md` | `{config_dir}/commands/{project}/target.md` |

## End
EOF
  printf '# Security\n' > "$dir/framework/rules/security-cross.md"
  printf '# Target\n' > "$dir/framework/commands/target.md"
  printf '# In\n' > "$dir/docs/in.md"
  ln -s in.md "$dir/docs/alias.md"
  printf '# Outside\n' > "$WORK/$1/repo2/secret.md"
  printf 'See https://github.com/acme/widget/blob/main/docs/alias.md here.\n' > "$dir/README.md"
  echo "$dir"
}

# track DIR — stage every file, as `git ls-files` reads the index.
track() {
  git -C "$1" init -q
  git -C "$1" add -A
}

# run_family DIR SCRIPT — run the fixture's copy of SCRIPT; sets out and code.
run_family() {
  set +e
  out="$("$BASH" "$1/scripts/audit/$2" 2> /dev/null)"
  code=$?
  set -e
}

# expect NAME CODE PATTERN — the last run exited CODE and its findings match
# PATTERN (an extended regex; empty means no finding at all).
expect() {
  if [ "$code" -ne "$2" ]; then fail "$1: exit $code, want $2: $out"; return; fi
  if [ -z "$3" ]; then
    if [ -n "$out" ]; then fail "$1: want no finding: $out"; else pass "$1"; fi
    return
  fi
  if printf '%s' "$out" | grep -qE -- "$3"; then pass "$1"; else fail "$1: no finding matched /$3/: $out"; fi
}

echo "Running audit path-containment tests..."

dir="$(make_tree a)"
track "$dir"
run_family "$dir" self-url-resolution.sh
expect "A: Family 36 passes regular files and a link inside the tree" 0 ""
run_family "$dir" manifest-destination-links.sh
expect "A: Family 35 passes regular files" 0 ""

dir="$(make_tree b)"
ln -s ../../repo2/secret.md "$dir/docs/out.md"
printf 'See https://github.com/acme/widget/blob/main/docs/out.md here.\n' > "$dir/README.md"
track "$dir"
run_family "$dir" self-url-resolution.sh
expect "B: Family 36 reports a URL naming a link out of the tree, a prefix-named sibling" 1 \
  '^self-url \| README\.md:1 \| absolute link into this repository names `docs/out\.md`, a symbolic link resolving outside the repository'

dir="$(make_tree c)"
ln -s ../../repo2/secret.md "$dir/docs/linked.md"
track "$dir"
run_family "$dir" self-url-resolution.sh
expect "C: Family 36 reports a tracked markdown file linking out of the tree, unread" 1 \
  '^self-url \| docs/linked\.md \| tracked markdown file is a symbolic link resolving outside the repository'

dir="$(make_tree d)"
rm "$dir/framework/rules/security-cross.md"
ln -s ../../../repo2/secret.md "$dir/framework/rules/security-cross.md"
track "$dir"
run_family "$dir" manifest-destination-links.sh
expect "D: Family 35a reports a Shared Files source linking out of the tree, uncopied" 1 \
  '^manifest-dest \| framework/bootstrap/ductus\.md \| Shared Files manifest source resolves outside the repository: framework/rules/security-cross\.md'

dir="$(make_tree e)"
rm "$dir/framework/commands/target.md"
ln -s ../../../repo2/secret.md "$dir/framework/commands/target.md"
track "$dir"
run_family "$dir" manifest-destination-links.sh
expect "E: Family 35b reports a slash-command source linking out of the tree, unread" 1 \
  '^manifest-dest \| framework/commands/target\.md \| slash-command manifest source resolves outside the repository'

if [ "$failures" -ne 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
echo "All audit path-containment cases passed."
