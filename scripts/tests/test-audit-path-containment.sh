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
#   F. Family 35a: a destination of exactly `..` is refused, not copied
#   G. Family 36: a URL path holding a percent-encoded NUL is unresolved
#   H. Family 36: an unreadable tracked file is reported with its reason
#
# "Not copied, probed or read" is asserted, not inferred: everything outside
# the tree would produce a finding of its own if it were — the outside file
# carries a self-URL to a missing path and two relative links that resolve
# nowhere, and the outside directory B names would be a `blob` URL naming a
# directory — so each case also requires that finding to be absent.
#
# Spec 026 task 7 extends the rule (026 plan, the decision "Every family
# contains the paths it did not write as a literal") to every other family
# that turns a path it did not write as a literal into a probe, a glob or an
# open: cases I to R and T to W, one per family, each putting a symbolic link
# resolving into the sibling `repo2` where the family would touch the path,
# with content there the family would report if it reached it, and requiring
# the containment finding and not that one:
#   I. Family 7 (sibling-coupling)           a spec.md and a plan.md, under a
#                                            configured spec root
#   J. Family 8 (introducing-drift)          a done spec.md, same root
#   K. Family 10 (migration-coverage)        procedure_file, a literal
#                                            target_path, a glob pattern, a
#                                            glob match, an orphan-scan match
#   L. Family 12 (fixture-session-shape)     a fixture session file
#   M. Family 17 (host-namespace-parity)     a registry config dir, its
#                                            commands directory, a namespace
#                                            entry, its prompts directory, a
#                                            prompt template
#   N. Family 24 (rename-sweep-residue)      a tracked markdown file
#   O. Family 27 (done-spec-criteria)        a tracked spec.md, the spec root
#   P. Family 37 (analyze-record-backlog)    a record, the spec root
#   Q. Family 39 (shipped-anchor-resolution) a tracked markdown file
#   R. Family 40 (deprecation-removal)       a tracked rule file
#   T. Family 4 (placeholder-roundtrip)     a command source and a reference
#                                            file a glob lists, and the command
#                                            file a reference inherits its
#                                            marker from
#   U. Family 9 (primitive-promotion)        a command source a glob lists
#   V. Family 16 (installer-command-parity)  a command source and a reference
#                                            file its globs list, and the
#                                            pointers read from them
#   W. Family 33 (readme-command-parity)     a command source a glob lists
#   S. a containment that cannot run: `contained` (scripts/audit/lib.sh) exits
#      0 inside, 1 outside, and 2 when it could not tell (no python3, one that
#      fails, one that prints nothing or no verdict), `contained_each` answers
#      the same per path of a list, and each family calling either, run where
#      python3 fails that call alone, reports the path as not checked and
#      stops, never reporting it as resolving outside (placed last, after T to W)
#
# The same file proves the audit fixes of 026's third review that a fixture
# reaches: I2 (Family 7 fails on an awk that cannot run match(), where it once
# exited 0 in silence), and L to L4 (Family 12 passes a fixture directory name
# to python as data, never as source; reports an absent fixtures directory;
# says on stderr that it found nothing to compare, and what it examined)
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
# one self-URL, and, beside the root, a directory `repo2` holding a file and a
# directory outside it, each of which a family would report if it reached it.
# Prints the directory.
make_tree() {
  local dir="$WORK/$1/repo"
  mkdir -p "$dir/scripts/audit" "$dir/framework/bootstrap" "$dir/framework/rules" \
    "$dir/framework/commands" "$dir/docs" "$dir/.ductus/bin" "$WORK/$1/repo2/dir"
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
  printf '# Outside\n\nSee https://github.com/acme/widget/blob/main/nope.md, [y](../y.md) and [z](sub/z.md).\n' \
    > "$WORK/$1/repo2/secret.md"
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

# run_family_err DIR SCRIPT — as run_family, and sets err to the family's
# stderr, where it says what it examined.
run_family_err() {
  set +e
  out="$("$BASH" "$1/scripts/audit/$2" 2> "$WORK/stderr")"
  code=$?
  err="$(cat "$WORK/stderr")"
  set -e
}

# expect_err NAME PATTERN — the last run_family_err's stderr matches PATTERN.
expect_err() {
  if printf '%s' "$err" | grep -qE -- "$2"; then pass "$1"; else fail "$1: stderr did not match /$2/: $err"; fi
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

# expect_absent NAME PATTERN — no finding of the last run matches PATTERN: the
# finding the family would make had it reached the outside path.
expect_absent() {
  if printf '%s' "$out" | grep -qE -- "$2"; then fail "$1: reached it, /$2/ matched: $out"; else pass "$1"; fi
}

echo "Running audit path-containment tests..."

dir="$(make_tree a)"
rm "$dir/framework/rules/security-cross.md"
ln -s ../../docs/in.md "$dir/framework/rules/security-cross.md"
track "$dir"
run_family "$dir" self-url-resolution.sh
expect "A: Family 36 passes regular files and a link inside the tree" 0 ""
run_family "$dir" manifest-destination-links.sh
expect "A: Family 35 passes a source that is a link inside the tree" 0 ""

dir="$(make_tree b)"
ln -s ../../repo2/dir "$dir/docs/out"
printf 'See https://github.com/acme/widget/blob/main/docs/out here.\n' > "$dir/README.md"
track "$dir"
run_family "$dir" self-url-resolution.sh
expect "B: Family 36 reports a URL naming a link out of the tree, a prefix-named sibling" 1 \
  '^self-url \| README\.md:1 \| absolute link into this repository names `docs/out`, a symbolic link resolving outside the repository'
expect_absent "B: Family 36 never probes it (a /blob/ URL naming its directory)" 'uses /blob/ for'

dir="$(make_tree c)"
ln -s ../../repo2/secret.md "$dir/docs/linked.md"
track "$dir"
run_family "$dir" self-url-resolution.sh
expect "C: Family 36 reports a tracked markdown file linking out of the tree" 1 \
  '^self-url \| docs/linked\.md \| tracked markdown file is a symbolic link resolving outside the repository'
expect_absent "C: Family 36 never reads it (its URL to a missing path)" 'names `nope\.md`'

dir="$(make_tree d)"
rm "$dir/framework/rules/security-cross.md"
ln -s ../../../repo2/secret.md "$dir/framework/rules/security-cross.md"
track "$dir"
run_family "$dir" manifest-destination-links.sh
expect "D: Family 35a reports a Shared Files source linking out of the tree" 1 \
  '^manifest-dest \| framework/bootstrap/ductus\.md \| Shared Files manifest source resolves outside the repository: framework/rules/security-cross\.md'
expect_absent "D: Family 35a never copies it (its links resolve nowhere)" 'shipped file links'

dir="$(make_tree e)"
rm "$dir/framework/commands/target.md"
ln -s ../../../repo2/secret.md "$dir/framework/commands/target.md"
track "$dir"
run_family "$dir" manifest-destination-links.sh
expect "E: Family 35b reports a slash-command source linking out of the tree" 1 \
  '^manifest-dest \| framework/commands/target\.md \| slash-command manifest source resolves outside the repository'
expect_absent "E: Family 35b never reads it (its links leave the command directory)" 'command source links'

dir="$(make_tree f)"
sed -i.bak 's#`specs/rules/security-cross.md`#`..`#' "$dir/framework/bootstrap/ductus.md"
rm "$dir/framework/bootstrap/ductus.md.bak"
track "$dir"
run_family "$dir" manifest-destination-links.sh
expect "F: Family 35a refuses a destination of exactly .." 1 \
  '^manifest-dest \| framework/bootstrap/ductus\.md \| Shared Files manifest destination escapes the target tree: \.\. \|'

dir="$(make_tree g)"
printf 'See https://github.com/acme/widget/blob/main/a%%00b here.\n' > "$dir/README.md"
track "$dir"
run_family "$dir" self-url-resolution.sh
expect "G: Family 36 reports a URL path holding a NUL as unresolved" 1 \
  '^self-url \| README\.md:1 \| absolute link into this repository names `a%00b`, which does not exist'

dir="$(make_tree h)"
track "$dir"
chmod 000 "$dir/docs/in.md"
if [ -r "$dir/docs/in.md" ]; then
  # A user who reads a mode-000 file (root) cannot build this case; say so
  # rather than count it as a pass.
  printf '  SKIP  H: this user can read a mode-000 file, so no unreadable file can be built\n'
else
  run_family "$dir" self-url-resolution.sh
  expect "H: Family 36 names an unreadable file's reason" 1 \
    '^self-url \| docs/in\.md \| tracked markdown file could not be read \(Permission denied\)'
fi
chmod 644 "$dir/docs/in.md"

# --- Spec 026 task 7: the rule across the remaining families -----------------

# new_tree CASE SCRIPT... — a fixture repository holding lib.sh and each named
# family script, the runtime as an adopter-owned pointer, a git repository, and
# beside it the sibling `repo2` the outward targets go in. Prints the
# directory.
new_tree() {
  local dir="$WORK/$1/repo" name
  mkdir -p "$dir/scripts/audit" "$dir/.ductus/bin" "$WORK/$1/repo2"
  shift
  cp "$REPO_ROOT/scripts/audit/lib.sh" "$dir/scripts/audit/"
  for name in "$@"; do
    cp "$REPO_ROOT/scripts/audit/$name" "$dir/scripts/audit/"
  done
  ln -s "$BIN" "$dir/.ductus/bin/ductus"
  git -C "$dir" init -q
  echo "$dir"
}

# far DIR — the sibling directory beside DIR, outside the repository.
far() { echo "$(dirname "$1")/repo2"; }

# Families 7 and 8 read feature directories the dashboard enumerates, under a
# spec root that is configured rather than `specs`.
configure_root() {
  printf '[paths]\nspecs-root = "features"\n' > "$1/.ductus/config.toml"
}

# I. Family 7 — a spec.md and a plan.md linking out, under a configured root.
dir="$(new_tree i sibling-coupling.sh)"
repo2="$(far "$dir")"
configure_root "$dir"
mkdir -p "$dir/features/001-aaa" "$dir/features/002-bbb"
printf -- '---\nstatus: draft\ndependencies: []\n---\n# A\n\nSee [b](../002-bbb/spec.md).\n' > "$dir/features/001-aaa/spec.md"
printf -- '---\nstatus: draft\ndependencies: []\n---\n# B\n\nSee [a](../001-aaa/spec.md).\n' > "$repo2/spec-b.md"
printf '# Plan\n\n## Affected Files\n\n| `x/shared.md` | edit |\n' > "$dir/features/001-aaa/plan.md"
cp "$dir/features/001-aaa/plan.md" "$repo2/plan-b.md"
ln -s "$repo2/spec-b.md" "$dir/features/002-bbb/spec.md"
ln -s "$repo2/plan-b.md" "$dir/features/002-bbb/plan.md"
track "$dir"
run_family "$dir" sibling-coupling.sh
expect "I: Family 7 reports a spec.md linking out of the tree, under a configured root" 1 \
  '^sibling-coupling \| features/002-bbb/spec\.md \| resolves outside the repository, so it was not read'
expect "I: Family 7 reports a plan.md linking out of the tree" 1 \
  '^sibling-coupling \| features/002-bbb/plan\.md \| resolves outside the repository, so it was not read'
expect_absent "I: Family 7 never reads them (the pair it would report as a bundling candidate)" 'bundling candidate'

# I2. Family 7 — an awk that cannot evaluate match() is a finding and a failing
# exit, not a silent pass: the probe runs at top level, where its drift counts.
# Two mutually linking draft specs, a plan for one only, so the family would
# otherwise find nothing to compare and exit 0.
dir="$(new_tree i2 sibling-coupling.sh)"
mkdir -p "$dir/specs/001-aaa" "$dir/specs/002-bbb" "$WORK/i2/bin-awk"
printf -- '---\nstatus: draft\ndependencies: []\n---\n# A\n\nSee [b](../002-bbb/spec.md).\n' > "$dir/specs/001-aaa/spec.md"
printf -- '---\nstatus: draft\ndependencies: []\n---\n# B\n\nSee [a](../001-aaa/spec.md).\n' > "$dir/specs/002-bbb/spec.md"
printf '# Plan\n\n## Affected Files\n\n| `x/shared.md` | edit |\n' > "$dir/specs/001-aaa/plan.md"
track "$dir"
run_family "$dir" sibling-coupling.sh
expect "I2: Family 7 passes the fixture with a working awk (control)" 0 ""
printf '#!/bin/sh\ncase "$*" in *%smatch("x"%s*) exit 1 ;; esac\nexec %s "$@"\n' "'" "'" "$(command -v awk)" > "$WORK/i2/bin-awk/awk"
chmod +x "$WORK/i2/bin-awk/awk"
set +e
out="$(PATH="$WORK/i2/bin-awk:$PATH" "$BASH" "$dir/scripts/audit/sibling-coupling.sh" 2> /dev/null)"
code=$?
set -e
expect "I2: Family 7 reports an awk that cannot run match() and exits non-zero" 1 \
  '^sibling-coupling \| \(precondition\) \| awk cannot evaluate match\(\)'

# J. Family 8 — a done spec.md linking out, under a configured root.
dir="$(new_tree j introducing-drift.sh)"
repo2="$(far "$dir")"
configure_root "$dir"
mkdir -p "$dir/features/001-old"
printf -- '---\nstatus: done\ndependencies: []\n---\n# Old\n\nRun `/capture` now.\n' > "$repo2/old.md"
ln -s "$repo2/old.md" "$dir/features/001-old/spec.md"
track "$dir"
run_family "$dir" introducing-drift.sh
expect "J: Family 8 reports a done spec.md linking out of the tree, under a configured root" 1 \
  '^introducing-drift \| features/001-old/spec\.md \| resolves outside the repository, so it was not read'
expect_absent "J: Family 8 never reads it (the old name it would report)" 'references old name'

# K. Family 10 — registry text naming paths that resolve outside the tree.
dir="$(new_tree k migration-coverage.sh)"
repo2="$(far "$dir")"
mkdir -p "$dir/framework/migrations" "$repo2/dir"
printf '# Gone\n' > "$repo2/dir/gone.md"
printf '# Match\n' > "$repo2/match.md"
printf '# Orphan\n' > "$repo2/orphan.md"
printf '# Real\n' > "$dir/framework/migrations/real.md"
ln -s "$repo2/dir" "$dir/framework/out"
ln -s "$repo2/match.md" "$dir/framework/lnkfile"
ln -s "$repo2/orphan.md" "$dir/framework/migrations/orphan.md"
cat > "$dir/framework/migrations.toml" << 'EOF'
[[migrations]]
id = "m-proc"
introduced_in = "0.1.0"
procedure_file = "framework/out/p.md"
target_paths = ["adopter/only.md"]

[[migrations]]
id = "m-target"
introduced_in = "0.1.0"
procedure_file = "framework/migrations/real.md"
target_paths = ["framework/out/gone.md", "framework/out/*", "framework/lnk*"]
EOF
track "$dir"
run_family "$dir" migration-coverage.sh
expect "K: Family 10 reports a procedure_file resolving outside the tree" 1 \
  "^migration-coverage \\| framework/migrations\\.toml \\| \\[\\[migrations\\]\\] entry 'm-proc' procedure_file resolves outside the repository and was not probed: framework/out/p\\.md"
expect "K: Family 10 reports a literal target_path resolving outside the tree" 1 \
  "entry 'm-target' target_path resolves outside the repository and was not probed: framework/out/gone\\.md"
expect "K: Family 10 reports a glob pattern resolving outside the tree before expanding it" 1 \
  "entry 'm-target' target_path resolves outside the repository and was not probed: framework/out/\\*"
expect "K: Family 10 reports a glob match resolving outside the tree" 1 \
  "entry 'm-target' target_path 'framework/lnk\\*' matched 'framework/lnkfile', which resolves outside the repository and was not probed"
expect "K: Family 10 reports an orphan-scan match resolving outside the tree" 1 \
  '^migration-coverage \| framework/migrations/orphan\.md \| procedure file resolves outside the repository'
expect_absent "K: Family 10 never probes them (a missing procedure_file)" 'references missing procedure_file'
expect_absent "K: Family 10 never probes them (a target_path that still exists)" 'was removed but'
expect_absent "K: Family 10 never probes them (an orphan procedure file)" 'orphan procedure file'

# L. Family 12 — a fixture session file linking out.
dir="$(new_tree l fixture-session-shape.sh)"
repo2="$(far "$dir")"
mkdir -p "$dir/runtime/tests/fixtures/f1/.ductus"
printf 'scenarioPath = "x"\n' > "$repo2/session.toml"
ln -s "$repo2/session.toml" "$dir/runtime/tests/fixtures/f1/.ductus/session.toml"
track "$dir"
run_family "$dir" fixture-session-shape.sh
expect "L: Family 12 reports a fixture session file linking out of the tree" 1 \
  '^fixture-session-shape \| runtime/tests/fixtures/f1/\.ductus/session\.toml \| fixture session file resolves outside the repository, so it was not read'
expect_absent "L: Family 12 never reads it (the legacy key it would report)" 'legacy camelCase'
run_family_err "$dir" fixture-session-shape.sh
expect_err "L: Family 12 says on stderr what it examined and what it did not read" \
  'examined 0 session file\(s\) under runtime/tests/fixtures, 1 not read'

# L2. Family 12 — a fixture directory name is committed text, passed to python
# as an argument and never spliced into its source. One name holds an
# apostrophe, which closed the old string and read as a parse failure; the
# other holds code that ran when the family was run.
dir="$(new_tree l2 fixture-session-shape.sh)"
mkdir -p "$dir/runtime/tests/fixtures/it's/.ductus" \
  "$dir/runtime/tests/fixtures/a').read());__import__('os').system('touch PWNED');x=(('b/.ductus"
# The code ran only when the text before it opened a file that existed and
# parsed, so a harmless file `a` sits beside the directory.
: > "$dir/runtime/tests/fixtures/a"
printf 'x = 1\n' > "$dir/runtime/tests/fixtures/it's/.ductus/session.toml"
printf 'x = 1\n' > "$dir/runtime/tests/fixtures/a').read());__import__('os').system('touch PWNED');x=(('b/.ductus/session.toml"
track "$dir"
run_family_err "$dir" fixture-session-shape.sh
expect "L2: Family 12 reads a fixture directory named with quotes and code as data" 0 ""
if [ -e "$dir/PWNED" ]; then fail "L2: Family 12 ran code from a fixture directory name"; else pass "L2: Family 12 ran no code from a fixture directory name"; fi
expect_err "L2: Family 12 examined both" 'examined 2 session file\(s\) under runtime/tests/fixtures, 0 not read'

# L3. Family 12 — an absent fixtures directory is a finding, not a clean run.
dir="$(new_tree l3 fixture-session-shape.sh)"
track "$dir"
run_family "$dir" fixture-session-shape.sh
expect "L3: Family 12 reports an absent fixtures directory" 1 \
  '^fixture-session-shape \| runtime/tests/fixtures \| the fixtures directory is absent'

# L4. Family 12 — a fixtures directory with no session file is nothing to
# compare, said on stderr, and not a failure.
dir="$(new_tree l4 fixture-session-shape.sh)"
mkdir -p "$dir/runtime/tests/fixtures/f1"
track "$dir"
run_family_err "$dir" fixture-session-shape.sh
expect "L4: Family 12 passes a fixtures directory holding no session file" 0 ""
expect_err "L4: Family 12 says there was nothing to compare" 'nothing to compare, no session file under runtime/tests/fixtures'

# M. Family 17 — the Agent Registry's config dir, and each entry beneath it.
# make_ns_tree CASE — a fixture whose registry names one config dir, `.agent`,
# and holds what the family asserts about the runtime; the case then builds
# the symbolic link under test, so the only installed namespace a family
# could find is the outward one, which matches no rendered namespace.
make_ns_tree() {
  local d
  d="$(new_tree "$1" host-namespace-parity.sh)"
  mkdir -p "$d/framework/bootstrap" "$d/framework/commands" "$d/runtime/src/schema"
  printf '| `key` | `name` | `config_dir` | `layout` |\n| --- | --- | --- | --- |\n| `a` | A | `.agent` | x |\n' \
    > "$d/framework/bootstrap/ductus.md"
  printf '%s\n' '/commands/{}/{command_name}.md' '/prompts/{}-{command_name}.md' 'project: Option<String>' > "$d/runtime/src/host.rs"
  printf '%s\n' '.ductus/config.toml' > "$d/runtime/src/schema/paths.rs"
  printf '# X\n' > "$d/framework/commands/x.md"
  printf '[host]\nproject = "ns"\n' > "$d/.ductus/config.toml"
  echo "$d"
}
# expect_ns NAME PATH — the last run reports PATH as resolving outside, and the
# namespace comparison it would otherwise make is absent.
expect_ns() {
  expect "$1" 1 "^host-namespace-parity \\| $2 \\| resolves outside the repository, so it was not probed"
  expect_absent "$1 (never compared)" 'matches no installed command namespace'
}

dir="$(make_ns_tree m1)"
repo2="$(far "$dir")"
mkdir -p "$repo2/agent/commands/wrong"
ln -s "$repo2/agent" "$dir/.agent"
track "$dir"
run_family "$dir" host-namespace-parity.sh
expect_ns "M: Family 17 reports a registry config dir linking out of the tree" '\.agent'

dir="$(make_ns_tree m2)"
repo2="$(far "$dir")"
mkdir -p "$repo2/cmds/wrong" "$dir/.agent"
ln -s "$repo2/cmds" "$dir/.agent/commands"
track "$dir"
run_family "$dir" host-namespace-parity.sh
expect_ns "M: Family 17 reports a commands directory linking out of the tree" '\.agent/commands'

dir="$(make_ns_tree m3)"
repo2="$(far "$dir")"
mkdir -p "$repo2/wrongdir" "$dir/.agent/commands"
ln -s "$repo2/wrongdir" "$dir/.agent/commands/wrong"
track "$dir"
run_family "$dir" host-namespace-parity.sh
expect_ns "M: Family 17 reports a namespace directory linking out of the tree" '\.agent/commands/wrong'

dir="$(make_ns_tree m4)"
repo2="$(far "$dir")"
mkdir -p "$repo2/prompts" "$dir/.agent"
printf '# P\n' > "$repo2/prompts/wrong-x.md"
ln -s "$repo2/prompts" "$dir/.agent/prompts"
track "$dir"
run_family "$dir" host-namespace-parity.sh
expect_ns "M: Family 17 reports a prompts directory linking out of the tree" '\.agent/prompts'

dir="$(make_ns_tree m5)"
repo2="$(far "$dir")"
mkdir -p "$dir/.agent/prompts"
printf '# P\n' > "$repo2/wrong-x.md"
ln -s "$repo2/wrong-x.md" "$dir/.agent/prompts/wrong-x.md"
track "$dir"
run_family "$dir" host-namespace-parity.sh
expect_ns "M: Family 17 reports a prompt template linking out of the tree" '\.agent/prompts/wrong-x\.md'

# N. Family 24 — a tracked markdown file linking out.
dir="$(new_tree n rename-sweep-residue.sh)"
repo2="$(far "$dir")"
mkdir -p "$dir/docs"
printf 'You should ductus this.\n' > "$repo2/residue.md"
printf '# Fine\n' > "$dir/docs/ok.md"
ln -s "$repo2/residue.md" "$dir/docs/lnk.md"
track "$dir"
run_family "$dir" rename-sweep-residue.sh
expect "N: Family 24 reports a tracked markdown file linking out of the tree" 1 \
  '^sweep-residue \| docs/lnk\.md \| tracked markdown file resolves outside the repository, so it was not read'
expect_absent "N: Family 24 never reads it (the residue it would report)" 'project name appears where a verb belongs'

# O. Family 27 — a tracked spec.md linking out, and a spec root resolving out.
dir="$(new_tree o done-spec-criteria.sh)"
repo2="$(far "$dir")"
mkdir -p "$dir/specs/001-ok" "$dir/specs/002-bad"
printf -- '---\nstatus: done\n---\n# Ok\n\n## Acceptance Criteria\n\n- [x] AC1: fine\n' > "$dir/specs/001-ok/spec.md"
printf -- '---\nstatus: done\n---\n# Bad\n\n## Acceptance Criteria\n\n- [ ] AC1: open\n' > "$repo2/spec.md"
ln -s "$repo2/spec.md" "$dir/specs/002-bad/spec.md"
track "$dir"
run_family "$dir" done-spec-criteria.sh
expect "O: Family 27 reports a tracked spec.md linking out of the tree" 1 \
  '^done-spec-criteria \| specs/002-bad/spec\.md:0 \| resolves outside the repository, so it was not read'
expect_absent "O: Family 27 never reads it (the unchecked criterion it would report)" 'is unchecked'

dir="$(new_tree o2 done-spec-criteria.sh)"
repo2="$(far "$dir")"
mkdir -p "$repo2/specs/001-bad"
printf '[paths]\nspecs-root = "../repo2/specs"\n' > "$dir/.ductus/config.toml"
track "$dir"
run_family "$dir" done-spec-criteria.sh
expect "O: Family 27 reports a spec root resolving outside the tree" 1 \
  '^done-spec-criteria \| \.\./repo2/specs/ \| the configured spec root resolves outside the repository'
expect_absent "O: Family 27 never lists under it (the empty enumeration it would report)" 'no tracked spec\.md files found'

# P. Family 37 — a record linking out, and a spec root linking out.
dir="$(new_tree p analyze-record-backlog.sh)"
repo2="$(far "$dir")"
mkdir -p "$dir/specs/001-x" "$dir/scripts/audit"
printf -- '---\nstatus: done\ndependencies: []\n---\n# X\n' > "$dir/specs/001-x/spec.md"
printf -- '---\nspec: 001-x\nlast-run: null\n---\n' > "$repo2/review.md"
ln -s "$repo2/review.md" "$dir/specs/001-x/review.md"
printf '0\n' > "$dir/scripts/audit/analyze-record-baseline.txt"
track "$dir"
run_family "$dir" analyze-record-backlog.sh
expect "P: Family 37 reports a record linking out of the tree" 1 \
  '^analyze-record \| specs/001-x/review\.md \| audit artifact resolves outside the repository, so it was not read'
expect_absent "P: Family 37 never reads it (the backlog it would count)" 'no analyze record'

dir="$(new_tree p2 analyze-record-backlog.sh)"
repo2="$(far "$dir")"
mkdir -p "$repo2/specs/001-x"
printf -- '---\nstatus: done\ndependencies: []\n---\n# X\n' > "$repo2/specs/001-x/spec.md"
ln -s "$repo2/specs" "$dir/linked"
printf '[paths]\nspecs-root = "linked"\n' > "$dir/.ductus/config.toml"
printf '0\n' > "$dir/scripts/audit/analyze-record-baseline.txt"
track "$dir"
run_family "$dir" analyze-record-backlog.sh
expect "P: Family 37 reports a spec root linking out of the tree" 1 \
  '^analyze-record \| linked/ \| the configured spec root resolves outside the repository'

# Q. Family 39 — a tracked markdown file linking out.
dir="$(new_tree q shipped-anchor-resolution.sh)"
repo2="$(far "$dir")"
mkdir -p "$dir/framework"
printf '# Constitution\n' > "$dir/framework/constitution.md"
printf '# Fine\n' > "$dir/framework/ok.md"
printf 'See §nosuchsection here.\n' > "$repo2/bad.md"
ln -s "$repo2/bad.md" "$dir/framework/lnk.md"
track "$dir"
run_family "$dir" shipped-anchor-resolution.sh
expect "Q: Family 39 reports a tracked markdown file linking out of the tree" 1 \
  '^shipped-anchors \| framework/lnk\.md \| tracked markdown file resolves outside the repository, so it was not read'
expect_absent "Q: Family 39 never hands it to the runtime (which refuses it, so the file is unexamined)" 'resolve-anchor produced no usable result'

# R. Family 40 — a tracked rule file linking out.
dir="$(new_tree r deprecation-removal.sh)"
repo2="$(far "$dir")"
mkdir -p "$dir/framework/rules"
printf '1.0.0\n' > "$dir/version"
printf '# Rules\n\n### X-AUTHN-001\n\n> A MUST b.\n' > "$dir/framework/rules/a-cross.md"
printf '# Rules\n\n### BE-AUTHN-009\n\n**DEPRECATED in 0.60.0:** moved elsewhere; removed in 0.62.0.\n\n> A MUST b.\n' > "$repo2/b.md"
ln -s "$repo2/b.md" "$dir/framework/rules/b-cross.md"
track "$dir"
run_family "$dir" deprecation-removal.sh
expect "R: Family 40 reports a tracked rule file linking out of the tree" 1 \
  '^deprecation-removal \| framework/rules/b-cross\.md \| tracked but resolves outside the repository, so its deprecation labels were not examined'
expect_absent "R: Family 40 never hands it to the runtime (which refuses it, so the run fails)" 'read-deprecation-labels exited non-zero'

# T. Family 4 (placeholder-roundtrip) — a command source and a reference file
# a glob lists, and the command file a reference inherits its marker from.
dir="$(new_tree t placeholder-roundtrip.sh)"
repo2="$(far "$dir")"
mkdir -p "$dir/framework/bootstrap" "$dir/framework/commands" "$dir/framework/reference"
printf '| `key` | `name` | `config_dir` | `layout` |\n| --- | --- | --- | --- |\n| `a` | A | `.agent` | x |\n' \
  > "$dir/framework/bootstrap/ductus.md"
printf '# Ok\n' > "$dir/framework/commands/ok.md"
printf 'Run /ductus:foo now.\n' > "$repo2/bad.md"
printf '<!-- audit:ignore-placeholders:file -->\n' > "$repo2/marker.md"
printf 'Run /ductus:baz now.\n' > "$repo2/ref.md"
ln -s "$repo2/bad.md" "$dir/framework/commands/lnk.md"
ln -s "$repo2/marker.md" "$dir/framework/commands/x.md"
printf 'Run /ductus:bar here.\n' > "$dir/framework/reference/x.md"
ln -s "$repo2/ref.md" "$dir/framework/reference/y.md"
track "$dir"
run_family "$dir" placeholder-roundtrip.sh
expect "T: Family 4 reports a command source linking out of the tree" 1 \
  '^placeholder-roundtrip \| framework/commands/lnk\.md \| resolves outside the repository, so it was not read'
expect "T: Family 4 reports a reference file linking out of the tree" 1 \
  '^placeholder-roundtrip \| framework/reference/y\.md \| resolves outside the repository, so it was not read'
expect_absent "T: Family 4 never reads them (the hardcoded names they hold)" 'lnk\.md:[0-9]+|y\.md:[0-9]+'
expect "T: Family 4 never reads a command file linking out for its file marker" 1 \
  '^placeholder-roundtrip \| framework/reference/x\.md:1 \| hardcoded `/ductus:`'

# U. Family 9 (primitive-promotion-candidates) — a command source a glob lists.
dir="$(new_tree u primitive-promotion-candidates.sh)"
repo2="$(far "$dir")"
mkdir -p "$dir/framework/commands"
printf 'foo\n' > "$dir/framework/runtime-tools.txt"
printf '## Instructions\n\n1. Run `foo`.\n' > "$dir/framework/commands/ok.md"
printf '## Instructions\n\n1. Do this in prose only.\n' > "$repo2/prose.md"
ln -s "$repo2/prose.md" "$dir/framework/commands/lnk.md"
track "$dir"
run_family "$dir" primitive-promotion-candidates.sh
expect "U: Family 9 reports a command source linking out of the tree" 1 \
  '^primitive-promotion \| framework/commands/lnk\.md \| resolves outside the repository, so it was not read'
expect_absent "U: Family 9 never reads it (the prose-only step it would report)" 'prose-only step'

# V. Family 16 (installer-command-parity) — a command source and a reference
# file its two globs list, each holding what the family would report.
dir="$(new_tree v installer-command-parity.sh)"
repo2="$(far "$dir")"
mkdir -p "$dir/framework/bootstrap" "$dir/framework/commands" "$dir/framework/reference" "$dir/scripts"
printf 'audit\n' > "$dir/scripts/maintainer-only-commands.txt"
printf '| Source | Destination |\n| --- | --- |\n| `framework/commands/ok.md` | `{config_dir}/commands/{project}/ok.md` |\n| `framework/reference/ok.md` | `.ductus/reference/ok.md` |\n' \
  > "$dir/framework/bootstrap/ductus.md"
printf 'See .ductus/reference/ok.md.\n' > "$dir/framework/commands/ok.md"
printf '# Ok\n' > "$dir/framework/reference/ok.md"
printf 'See .ductus/reference/zzz.md.\n' > "$repo2/cmd.md"
printf '# Orphan\n' > "$repo2/ref.md"
ln -s "$repo2/cmd.md" "$dir/framework/commands/lnk.md"
ln -s "$repo2/ref.md" "$dir/framework/reference/lnkr.md"
track "$dir"
run_family "$dir" installer-command-parity.sh
expect "V: Family 16 reports a command source linking out of the tree" 1 \
  '^installer-command-parity \| framework/commands/lnk\.md \| resolves outside the repository, so it was not read'
expect "V: Family 16 reports a reference file linking out of the tree" 1 \
  '^installer-command-parity \| framework/reference/lnkr\.md \| resolves outside the repository, so it was not read'
expect_absent "V: Family 16 never counts a command it could not read (no manifest row)" 'has no installer manifest row'
expect_absent "V: Family 16 never counts a reference it could not read (no command)" 'reference file has no command'
expect_absent "V: Family 16 never reads a pointer from it (a file no row installs)" 'names \.ductus/reference/zzz'

# W. Family 33 (readme-command-parity) — a command source a glob lists.
dir="$(new_tree w readme-command-parity.sh)"
repo2="$(far "$dir")"
mkdir -p "$dir/framework/commands" "$dir/scripts"
printf 'audit\n' > "$dir/scripts/maintainer-only-commands.txt"
printf 'The `/ok` command.\n' > "$dir/README.md"
printf '# Ok\n' > "$dir/framework/commands/ok.md"
printf '# Lnk\n' > "$repo2/lnk.md"
ln -s "$repo2/lnk.md" "$dir/framework/commands/lnk.md"
track "$dir"
run_family "$dir" readme-command-parity.sh
expect "W: Family 33 reports a command source linking out of the tree" 1 \
  '^readme-command \| framework/commands/lnk\.md \| resolves outside the repository, so it was not probed'
expect_absent "W: Family 33 never counts it (a command the README omits)" 'is shipped to adopters but appears nowhere'

# S. A containment that cannot run is reported as that, never as outside.
s_dir="$WORK/s"
mkdir -p "$s_dir/base/in" "$s_dir/base2" "$s_dir/bin-none" "$s_dir/bin-fail" \
  "$s_dir/bin-silent" "$s_dir/bin-garbled" "$s_dir/bin-uncontaining"
ln -s ../base2 "$s_dir/base/out"
printf '#!/bin/sh\nexit 1\n' > "$s_dir/bin-fail/python3"
printf '#!/bin/sh\nexit 0\n' > "$s_dir/bin-silent/python3"
printf '#!/bin/sh\necho yes\n' > "$s_dir/bin-garbled/python3"
# The interpreter itself, not the first python3 on PATH, which may be a shim.
real_python="$(python3 -c 'import sys; print(sys.executable)')"
cat > "$s_dir/bin-uncontaining/python3" << EOF
#!/bin/sh
# Fails the containment call (contained in lib.sh, whose program is its third
# argument) and passes every other call to the interpreter.
case "\$3" in
  *'print("inside"'*) exit 1 ;;
esac
exec "$real_python" "\$@"
EOF
chmod +x "$s_dir"/bin-*/python3

# verdict NAME WANT [PATH] PATH-ARG — contained's exit status for PATH-ARG
# under BASE, with PATH set after lib.sh is sourced when given.
verdict() {
  local rc
  set +e
  if [ -n "$3" ]; then
    ( . "$REPO_ROOT/scripts/audit/lib.sh" && PATH="$3" && contained "$s_dir/base" "$4" )
  else
    ( . "$REPO_ROOT/scripts/audit/lib.sh" && contained "$s_dir/base" "$4" )
  fi
  rc=$?
  set -e
  if [ "$rc" -eq "$2" ]; then pass "S: contained $1"; else fail "S: contained $1: exit $rc, want $2"; fi
}
verdict "answers 0 for a path inside" 0 "" "$s_dir/base/in"
verdict "answers 1 for a link resolving outside" 1 "" "$s_dir/base/out"
verdict "answers 1 for a sibling whose name the base is a prefix of" 1 "" "$s_dir/base2"
verdict "answers 2 with no python3 on PATH" 2 "$s_dir/bin-none" "$s_dir/base/in"
verdict "answers 2 for a python3 that fails" 2 "$s_dir/bin-fail" "$s_dir/base/in"
verdict "answers 2 for a python3 that prints nothing" 2 "$s_dir/bin-silent" "$s_dir/base/in"
verdict "answers 2 for a python3 that prints no verdict" 2 "$s_dir/bin-garbled" "$s_dir/base/in"

# each_verdict NAME WANT PATH-OVERRIDE — contained_each over an inside path and
# a link resolving outside, one verdict line each, with PATH-OVERRIDE ahead of
# the real PATH when given. The no-python3 case runs with only the tools
# contained_each calls on PATH, linked into bin-tools.
mkdir -p "$s_dir/bin-tools"
for tool in grep wc tr; do ln -s "$(command -v "$tool")" "$s_dir/bin-tools/$tool"; done
each_verdict() {
  local got
  if [ "$3" = "$s_dir/bin-none" ]; then
    got="$( . "$REPO_ROOT/scripts/audit/lib.sh" && PATH="$s_dir/bin-tools" && printf '%s\n%s\n' "$s_dir/base/in" "$s_dir/base/out" | contained_each "$s_dir/base" )"
  elif [ -n "$3" ]; then
    got="$( . "$REPO_ROOT/scripts/audit/lib.sh" && PATH="$3:$PATH" && printf '%s\n%s\n' "$s_dir/base/in" "$s_dir/base/out" | contained_each "$s_dir/base" )"
  else
    got="$( . "$REPO_ROOT/scripts/audit/lib.sh" && printf '%s\n%s\n' "$s_dir/base/in" "$s_dir/base/out" | contained_each "$s_dir/base" )"
  fi
  if [ "$got" = "$2" ]; then pass "S: contained_each $1"; else fail "S: contained_each $1: got $got, want $2"; fi
}
tab="$(printf '\t')"
each_verdict "answers inside and outside, one line per path" \
  "inside${tab}$s_dir/base/in
outside${tab}$s_dir/base/out" ""
unchecked_both="unchecked${tab}$s_dir/base/in
unchecked${tab}$s_dir/base/out"
each_verdict "leaves every path unchecked with no python3 on PATH" "$unchecked_both" "$s_dir/bin-none"
each_verdict "leaves every path unchecked for a python3 that fails" "$unchecked_both" "$s_dir/bin-fail"
each_verdict "leaves every path unchecked for a python3 that prints nothing" "$unchecked_both" "$s_dir/bin-silent"
each_verdict "leaves every path unchecked for a python3 that prints no verdict" "$unchecked_both" "$s_dir/bin-garbled"

# Each family calling contained, over the fixture its own case built, where
# python3 fails the containment call alone.
for entry in i:sibling-coupling.sh:sibling-coupling j:introducing-drift.sh:introducing-drift \
  k:migration-coverage.sh:migration-coverage l:fixture-session-shape.sh:fixture-session-shape \
  m1:host-namespace-parity.sh:host-namespace-parity n:rename-sweep-residue.sh:sweep-residue \
  o:done-spec-criteria.sh:done-spec-criteria p:analyze-record-backlog.sh:analyze-record \
  r:deprecation-removal.sh:deprecation-removal d:manifest-destination-links.sh:manifest-dest \
  t:placeholder-roundtrip.sh:placeholder-roundtrip u:primitive-promotion-candidates.sh:primitive-promotion \
  v:installer-command-parity.sh:installer-command-parity w:readme-command-parity.sh:readme-command; do
  case_dir="$WORK/${entry%%:*}/repo"
  script="${entry#*:}"
  label="${script#*:}"
  script="${script%%:*}"
  set +e
  out="$(PATH="$s_dir/bin-uncontaining:$PATH" "$BASH" "$case_dir/scripts/audit/$script" 2> /dev/null)"
  code=$?
  set -e
  expect "S: $script reports a path it could not contain as not checked" 1 \
    "^$label \\| [^|]+ \\| could not tell whether this path resolves inside the repository"
  expect_absent "S: $script never reports it as resolving outside" 'resolves outside the repository'
  if [ "$(printf '%s\n' "$out" | grep -c .)" -eq 1 ]; then
    pass "S: $script stops after it"
  else
    fail "S: $script stops after it: $out"
  fi
done

if [ "$failures" -ne 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
echo "All audit path-containment cases passed."
