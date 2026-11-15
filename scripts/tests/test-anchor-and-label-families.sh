#!/usr/bin/env bash
# Test surface for the file form of /audit Families 39 and 40
# (scripts/audit/shipped-anchor-resolution.sh and
# scripts/audit/deprecation-removal.sh; spec 067 task 35, BE-PAYLOAD-001).
#
# `resolve-anchor` and `read-deprecation-labels` return their lists inline
# while the result fits `[artifacts] read-size-bytes` and past it in the file
# their `lists-file` names. Each family renders the lists, so in the file form
# it must read them there, and a file it cannot read, or one lacking a list,
# must be a `(precondition)` finding that says nothing was checked, never an
# empty result.
#
# Coverage, for each of Family 39 and Family 40:
#   A. inline form: a fixture with one defect is reported, and no list file is
#      written
#   B. file form: the same fixture with a read size too small for the result
#      reports the same findings, and the list file exists, so the file form
#      ran rather than the inline one
#   C. a result naming a list file that does not exist is a `(precondition)`
#      finding and reports nothing as checked
#   D. a result naming a list file that lacks a list is a `(precondition)`
#      finding too
#
# Needs the release runtime (runtime/target/release/ductus), as
# framework-checks.yml builds it before this step.
#
# Usage: scripts/tests/test-anchor-and-label-families.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
BIN="$REPO_ROOT/runtime/target/release/ductus"

if [ ! -x "$BIN" ]; then
  echo "no release runtime at $BIN — build it (cargo build --release in runtime/) before this test" >&2
  exit 1
fi

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# make_tree FAMILY CASE — a fixture git repository holding the family script,
# its lib, the runtime as an adopter-owned pointer, and the files the family
# enumerates: for Family 39 the constitution declaring `§known` and a document
# citing `§unknown`; for Family 40 a `version` of 0.62.0 and a rule file whose
# deprecation label names removal in 0.60.0, which is due. Prints the
# directory.
make_tree() {
  local script="$1" dir="$WORK/$1-$2/repo"
  mkdir -p "$dir/scripts/audit" "$dir/framework/rules" "$dir/.ductus/bin"
  cp "$REPO_ROOT/scripts/audit/lib.sh" "$REPO_ROOT/scripts/audit/$script.sh" "$dir/scripts/audit/"
  ln -s "$BIN" "$dir/.ductus/bin/ductus"
  case "$script" in
    shipped-anchor-resolution)
      printf '# C\n\n<!-- §known -->\n' > "$dir/framework/constitution.md"
      printf '# D\n\nSee §known and §unknown.\n' > "$dir/framework/doc.md"
      ;;
    deprecation-removal)
      printf '0.62.0\n' > "$dir/version"
      printf '# Rules\n\n### BE-OLD-001\n\n**DEPRECATED in 0.50.0:** moved; removed in 0.60.0.\n\n> A MUST.\n' \
        > "$dir/framework/rules/r.md"
      ;;
  esac
  (cd "$dir" && git init -q . && git add -A . > /dev/null 2>&1)
  printf '%s\n' "$dir"
}

# run_family SCRIPT DIR — run the fixture's copy of the family; sets out, code.
run_family() {
  code=0
  out="$(cd "$2" && "$BASH" "scripts/audit/$1.sh" 2> /dev/null)" || code=$?
}

# stub DIR BODY — replace the fixture's runtime with a script printing BODY.
stub() {
  rm -f "$1/.ductus/bin/ductus"
  printf '#!/usr/bin/env bash\ncat << '"'"'JSON'"'"'\n%s\nJSON\n' "$2" > "$1/.ductus/bin/ductus"
  chmod +x "$1/.ductus/bin/ductus"
}

# family SCRIPT LABEL EXPECT RESULTS_DIR STUB_FILE_MISSING STUB_FILE_LACKING
# LACKING_CONTENT — the four cases for one family.
family() {
  local script="$1" label="$2" expect="$3" results="$4" missing_stub="$5" lacking_stub="$6" lacking_body="$7"
  local dir inline_out

  echo "$label A. inline form"
  dir="$(make_tree "$script" inline)"
  run_family "$script" "$dir"
  inline_out="$out"
  if [ "$code" = 1 ] && printf '%s' "$out" | grep -q "$expect"; then
    pass "the defect is reported"
  else
    fail "$label inline form: exit $code, output: $out"
  fi
  if [ ! -e "$dir/.ductus/results" ]; then
    pass "no list file is written under the read size"
  else
    fail "$label inline form wrote a list file"
  fi

  echo "$label B. file form"
  dir="$(make_tree "$script" file)"
  printf '[artifacts]\nread-size-bytes = 1\n' > "$dir/.ductus/config.toml"
  run_family "$script" "$dir"
  if [ -n "$(find "$dir/.ductus/results/$results" -name '*.json' 2> /dev/null)" ]; then
    pass "the result moved its lists to a file"
  else
    fail "$label file form: no list file was written"
  fi
  if [ "$code" = 1 ] && [ "$out" = "$inline_out" ]; then
    pass "the file form reports what the inline form reports"
  else
    fail "$label file form: exit $code, output: $out"
  fi

  echo "$label C. a missing list file"
  dir="$(make_tree "$script" missing)"
  stub "$dir" "$missing_stub"
  run_family "$script" "$dir"
  if [ "$code" = 1 ] && printf '%s' "$out" | grep -q '(precondition)' \
    && printf '%s' "$out" | grep -q 'moved its lists to a file that could not be read'; then
    pass "an unreadable list file is a precondition finding"
  else
    fail "$label missing list file: exit $code, output: $out"
  fi

  echo "$label D. a list file lacking a list"
  dir="$(make_tree "$script" lacking)"
  mkdir -p "$dir/.ductus/results"
  printf '%s\n' "$lacking_body" > "$dir/.ductus/results/partial.json"
  stub "$dir" "$lacking_stub"
  run_family "$script" "$dir"
  if [ "$code" = 1 ] && printf '%s' "$out" | grep -q '(precondition)' \
    && printf '%s' "$out" | grep -q 'it holds no'; then
    pass "a list file missing a list is a precondition finding"
  else
    fail "$label list file lacking a list: exit $code, output: $out"
  fi
}

family shipped-anchor-resolution "Family 39" "framework/doc.md" "resolve-anchor" \
  '{"form":"file","lists-file":".ductus/results/resolve-anchor/doc/absent.json","references-count":2,"unresolved-count":1,"qualified":0,"intra-document":0,"cross-document":0,"anchor-target-unjudgeable":0,"anchor-target-unreadable":0}' \
  '{"form":"file","lists-file":".ductus/results/partial.json","references-count":0,"unresolved-count":0,"qualified":0,"intra-document":0,"cross-document":0,"anchor-target-unjudgeable":0,"anchor-target-unreadable":0}' \
  '{"references": []}'

family deprecation-removal "Family 40" "BE-OLD-001 is deprecated" "read-deprecation-labels" \
  '{"form":"file","lists-file":".ductus/results/read-deprecation-labels/paths/absent.json","labels-count":1,"unreadable-count":0,"examined":1}' \
  '{"form":"file","lists-file":".ductus/results/partial.json","labels-count":0,"unreadable-count":0,"examined":1}' \
  '{"labels": []}'

if [ "$failures" -gt 0 ]; then
  echo "$failures failure(s)" >&2
  exit 1
fi
echo "all passed"
