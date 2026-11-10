#!/usr/bin/env bash
# Regenerate .claude/commands/ductus/*.md from framework/commands/*.md and
# framework/bootstrap/configure/claude.md, and .ductus/reference/*.md from
# framework/reference/*.md.
#
# Substitutes {project} -> ductus and {cli-config-dir} -> .claude.
# The configure command is sourced from framework/bootstrap/configure/claude.md.
# Files in .claude/commands/ductus/ that do not correspond to a current source
# are removed so renames flow through cleanly.
#
# The Claude copies are built by the runtime's apply-manifest, as /ductus
# builds an adopter's, with tool-search-line set on each (spec 080), so the
# runtime must be reachable: DUCTUS_BIN, else runtime/target/release/ductus.
#
# Reference pass (spec 071): each command's reference is one
# file shared by every agent, so it lands at .ductus/reference/<name>.md — the
# path an adopter's /ductus installs it to — with {project} substituted and no
# per-agent value. A copy with no source is removed, as above.
#
# Flags:
#   --check    Compare generated content against the current destination;
#              exit 0 when in sync, 1 when drift exists (prints a unified
#              diff to stdout) or when the runtime cannot build the Claude
#              copies (unreachable, predating tool-search-line, or failing
#              apply-manifest). Used by `/audit`'s check-zero precondition
#              pass to surface generator drift without writing.

set -euo pipefail

check_mode=0
for arg in "$@"; do
  case "$arg" in
    --check) check_mode=1 ;;
    -h|--help)
      sed -n '2,26p' "$0" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *)
      echo "unknown argument: $arg" >&2
      exit 2
      ;;
  esac
done

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC="$ROOT/framework/commands"
CONFIGURE_SRC="$ROOT/framework/bootstrap/configure/claude.md"
DEST="$ROOT/.claude/commands/ductus"

PROJECT="ductus"
CONFIG_DIR=".claude"

# Pi pass (spec 064): the same command set as flat project-hyphenated
# prompt templates under .pi/prompts/, plus the verbatim bridge extension.
# Discovery is flat and filename-keyed, so the `{project}-{name}` prefix is
# what keeps the shared prompts/ directory collision-free.
PI_SRC="$ROOT/framework/bootstrap/configure/pi.md"
PI_DEST="$ROOT/.pi/prompts"
PI_EXT_DEST="$ROOT/.pi/extensions/ductus.ts"
PI_BRIDGE_SRC="$ROOT/framework/bootstrap/pi/ductus-bridge.ts"

PI_CONFIG_DIR=".pi"

REF_SRC="$ROOT/framework/reference"
REF_DEST="$ROOT/.ductus/reference"

substitute() {
  sed -e "s/{project}/$PROJECT/g" -e "s|{cli-config-dir}|$1|g"
}

# A shared file has no per-agent value, so only {project} is substituted.
substitute_shared() {
  sed -e "s/{project}/$PROJECT/g"
}

# The runtime that builds the Claude copies. The pre-commit hook passes the one
# it resolved as DUCTUS_BIN; a relative path is taken from where this started.
ductus_bin="${DUCTUS_BIN:-$ROOT/runtime/target/release/ductus}"
case "$ductus_bin" in
  /*) ;;
  */*) ductus_bin="$PWD/$ductus_bin" ;;
esac
if ! command -v "$ductus_bin" > /dev/null 2>&1; then
  echo "gen-claude-commands: the ductus runtime is not reachable at $ductus_bin." >&2
  echo "  Build it with (cd runtime && cargo build --release), or set DUCTUS_BIN." >&2
  exit 1
fi
# Every wait on the runtime is bounded (BE-TIMEOUT-001): it is a local,
# deterministic process, so a call that outlasts the bound is hung, and the
# pre-commit hook that runs this would otherwise wait on it with no end.
# DUCTUS_GEN_TIMEOUT_S overrides the bound, in whole seconds.
runtime_timeout="${DUCTUS_GEN_TIMEOUT_S:-120}"
refuse_timeout() {
  echo "gen-claude-commands: DUCTUS_GEN_TIMEOUT_S must be a positive whole number of seconds, got '$runtime_timeout'." >&2
  exit 1
}
case "$runtime_timeout" in
  '' | *[!0-9]*) refuse_timeout ;;
esac
# All digits, so a decimal comparison; `00` is zero too.
if [ "$runtime_timeout" -eq 0 ]; then refuse_timeout; fi
# Run "$@" as a background job that a watchdog stops once the bound passes,
# since macOS ships no timeout(1). The watchdog holds none of the caller's
# output, so a command substitution around this does not wait on its sleep.
bounded() {
  local pid rc=0 watchdog
  "$@" &
  pid=$!
  (
    sleep "$runtime_timeout" &
    nap=$!
    trap 'kill "$nap" 2>/dev/null; exit 0' TERM
    wait "$nap"
    kill "$pid" 2>/dev/null
  ) < /dev/null > /dev/null 2>&1 &
  watchdog=$!
  wait "$pid" || rc=$?
  kill "$watchdog" 2>/dev/null || true
  wait "$watchdog" 2>/dev/null || true
  if [ "$rc" -eq 143 ]; then
    echo "gen-claude-commands: $1 did not answer within ${runtime_timeout}s and was stopped." >&2
  fi
  return "$rc"
}
# A runtime that predates tool-search-line (spec 080) ignores the field, so it
# would write every Claude copy without its ToolSearch line, and the pre-commit
# hook would commit them: 81285165 did. Refuse it before anything is written.
# The help is read whole first: `grep -q` in a pipeline under pipefail can
# fail a match by closing the pipe early.
if ! manifest_help="$(bounded "$ductus_bin" apply-manifest --help 2>&1)"; then
  echo "gen-claude-commands: $ductus_bin apply-manifest --help failed:" >&2
  printf '%s\n' "$manifest_help" >&2
  exit 1
fi
case "$manifest_help" in
  *tool-search-line*) ;;
  *)
    echo "gen-claude-commands: the runtime at $ductus_bin predates apply-manifest's tool-search-line." >&2
    echo "  It would write the Claude copies without their ToolSearch line." >&2
    echo "  Rebuild it with (cd runtime && cargo build --release), or set DUCTUS_BIN." >&2
    exit 1
    ;;
esac

# Write apply-manifest's entries and substitutions for the Claude copies into
# directory $1: one entry per command source, and configure.md from Claude's
# permission file, each with tool-search-line set.
write_claude_manifest() {
  local dir="$1" first=1 name
  {
    printf '['
    for src in "$SRC"/*.md; do
      name="$(basename "$src")"
      if [ "$first" -eq 0 ]; then printf ','; fi
      first=0
      printf '{"source":"commands/%s","dest":"%s/commands/%s/%s","strategy":"update","tool-search-line":true}' \
        "$name" "$CONFIG_DIR" "$PROJECT" "$name"
    done
    printf ',{"source":"bootstrap/configure/claude.md","dest":"%s/commands/%s/configure.md","strategy":"update","tool-search-line":true}]\n' \
      "$CONFIG_DIR" "$PROJECT"
  } > "$dir/entries.json"
  printf '{"project":"%s","cli-config-dir":"%s"}\n' "$PROJECT" "$CONFIG_DIR" \
    > "$dir/substitutions.json"
}

# Build the Claude copies under $1/$CONFIG_DIR/commands/$PROJECT/ from the
# sources under $1/framework/, with the manifest files in $2. The runtime runs
# in $1, since apply-manifest holds both roots inside the directory it runs in.
build_claude_copies() {
  (
    cd "$1"
    bounded "$ductus_bin" apply-manifest --source-root framework --target-root . \
      --entries-json "$2/entries.json" --substitutions-json "$2/substitutions.json"
  ) > /dev/null
}

# Track expected destination filenames so we can prune obsolete generated files.
expected=()

if [ "$check_mode" -eq 1 ]; then
  # --check mode: generate into a tempdir, diff against DEST, report drift
  # without modifying anything on disk.
  tmpdir="$(mktemp -d -t gen-claude-commands-check-XXXXXX)"
  trap 'rm -rf "$tmpdir"' EXIT
  # The Claude copies, built over a staged copy of their sources.
  claudetmp="$tmpdir/claude"
  mkdir -p "$claudetmp/framework/commands" "$claudetmp/framework/bootstrap/configure" \
    "$claudetmp/framework/reference"
  cp "$SRC"/*.md "$claudetmp/framework/commands/"
  # apply-manifest reads the references the commands cite (spec 080).
  cp "$REF_SRC"/*.md "$claudetmp/framework/reference/"
  cp "$CONFIGURE_SRC" "$claudetmp/framework/bootstrap/configure/claude.md"
  write_claude_manifest "$tmpdir"
  build_claude_copies "$claudetmp" "$tmpdir"
  claude_out="$claudetmp/$CONFIG_DIR/commands/$PROJECT"
  for src in "$SRC"/*.md; do
    expected+=("$(basename "$src")")
  done
  expected+=("configure.md")

  # Pi pass: flat `{project}-{name}` prompt templates in a pi tempdir, plus
  # the bridge. The prompt-template body carries the same substitutions as
  # the claude copy (with {cli-config-dir} -> .pi); the bridge is verbatim.
  pitmp="$tmpdir/pi"
  mkdir -p "$pitmp"
  pidi_expected=()
  for src in "$SRC"/*.md; do
    name="$(basename "$src")"
    substitute "$PI_CONFIG_DIR" < "$src" > "$pitmp/$PROJECT-$name"
    pidi_expected+=("$PROJECT-$name")
  done
  substitute "$PI_CONFIG_DIR" < "$PI_SRC" > "$pitmp/$PROJECT-configure.md"
  pidi_expected+=("$PROJECT-configure.md")
  tmp_bridge="$pitmp/ductus.ts"
  cp "$PI_BRIDGE_SRC" "$tmp_bridge"

  # Reference pass: the shared .ductus/reference/ copies.
  reftmp="$tmpdir/reference"
  mkdir -p "$reftmp"
  ref_expected=()
  for src in "$REF_SRC"/*.md; do
    [ -f "$src" ] || continue
    name="$(basename "$src")"
    substitute_shared < "$src" > "$reftmp/$name"
    ref_expected+=("$name")
  done

  drift=0
  # Compare every expected file against DEST.
  for name in "${expected[@]}"; do
    src_path="$claude_out/$name"
    dest_path="$DEST/$name"
    if [ ! -f "$dest_path" ]; then
      echo "missing in DEST: $name"
      drift=1
      continue
    fi
    if ! diff -q "$src_path" "$dest_path" >/dev/null 2>&1; then
      echo "drift in $name:"
      diff -u "$dest_path" "$src_path" || true
      drift=1
    fi
  done
  # Compare pi prompt templates against the pi destination.
  for name in "${pidi_expected[@]}"; do
    src_path="$pitmp/$name"
    dest_path="$PI_DEST/$name"
    if [ ! -f "$dest_path" ]; then
      echo "missing in $PI_DEST: $name"
      drift=1
      continue
    fi
    if ! diff -q "$src_path" "$dest_path" >/dev/null 2>&1; then
      echo "drift in $name:"
      diff -u "$dest_path" "$src_path" || true
      drift=1
    fi
  done
  # Compare the reference copies against .ductus/reference/.
  for name in ${ref_expected[@]+"${ref_expected[@]}"}; do
    src_path="$reftmp/$name"
    dest_path="$REF_DEST/$name"
    if [ ! -f "$dest_path" ]; then
      echo "missing in $REF_DEST: $name"
      drift=1
      continue
    fi
    if ! diff -q "$src_path" "$dest_path" >/dev/null 2>&1; then
      echo "drift in $name:"
      diff -u "$dest_path" "$src_path" || true
      drift=1
    fi
  done
  # Compare the pi bridge extension (verbatim).
  if ! diff -q "$tmp_bridge" "$PI_EXT_DEST" >/dev/null 2>&1; then
    echo "drift in $PI_EXT_DEST:"
    diff -u "$PI_EXT_DEST" "$tmp_bridge" || true
    drift=1
  fi
  # Detect orphans — files in DEST that no longer have a source.
  for existing in "$DEST"/*.md; do
    name="$(basename "$existing")"
    keep=0
    for e in "${expected[@]}"; do
      if [ "$name" = "$e" ]; then keep=1; break; fi
    done
    if [ "$keep" -eq 0 ]; then
      echo "orphan in DEST (no source): $name"
      drift=1
    fi
  done
  # Detect orphans in the pi prompts directory — files matching the
  # `{project}-*.md` namespace that no longer have a source. Foreign prompt
  # templates (outside the `{project}-` namespace) are never touched: the
  # flat directory is shared, and the default `*.md` glob would delete the
  # adopter's own files (spec 064 D1).
  if [ -d "$PI_DEST" ]; then
    for existing in "$PI_DEST"/"$PROJECT"-*.md; do
      [ -f "$existing" ] || continue
      name="$(basename "$existing")"
      keep=0
      for e in "${pidi_expected[@]}"; do
        if [ "$name" = "$e" ]; then keep=1; break; fi
      done
      if [ "$keep" -eq 0 ]; then
        echo "orphan in $PI_DEST (no source): $name"
        drift=1
      fi
    done
  fi
  # Detect orphans in the reference directory — copies with no source.
  if [ -d "$REF_DEST" ]; then
    for existing in "$REF_DEST"/*.md; do
      [ -f "$existing" ] || continue
      name="$(basename "$existing")"
      keep=0
      for e in ${ref_expected[@]+"${ref_expected[@]}"}; do
        if [ "$name" = "$e" ]; then keep=1; break; fi
      done
      if [ "$keep" -eq 0 ]; then
        echo "orphan in $REF_DEST (no source): $name"
        drift=1
      fi
    done
  fi
  exit "$drift"
fi

# Write mode (default): regenerate destination from source.

# One command per source file in framework/commands/, and configure.md from
# the agent-specific permission file, built in place by apply-manifest.
manifest_dir="$(mktemp -d -t gen-claude-commands-XXXXXX)"
trap 'rm -rf "$manifest_dir"' EXIT
write_claude_manifest "$manifest_dir"
build_claude_copies "$ROOT" "$manifest_dir"
for src in "$SRC"/*.md; do
  expected+=("$(basename "$src")")
done
expected+=("configure.md")

# Pi pass (write mode): flat `{project}-{name}` prompt templates + the bridge.
mkdir -p "$PI_DEST" "$(dirname "$PI_EXT_DEST")"
pi_expected=()
for src in "$SRC"/*.md; do
  name="$(basename "$src")"
  substitute "$PI_CONFIG_DIR" < "$src" > "$PI_DEST/$PROJECT-$name"
  pi_expected+=("$PROJECT-$name")
done
substitute "$PI_CONFIG_DIR" < "$PI_SRC" > "$PI_DEST/$PROJECT-configure.md"
pi_expected+=("$PROJECT-configure.md")
cp "$PI_BRIDGE_SRC" "$PI_EXT_DEST"

# Reference pass (write mode): the shared .ductus/reference/ copies.
mkdir -p "$REF_DEST"
ref_expected=()
for src in "$REF_SRC"/*.md; do
  [ -f "$src" ] || continue
  name="$(basename "$src")"
  substitute_shared < "$src" > "$REF_DEST/$name"
  ref_expected+=("$name")
done

# Prune any .md files in DEST that are no longer in the expected set.
# No exceptions: every file under .claude/commands/ductus/ is generated, from
# framework/commands/ or, for configure.md, from the agent's permission file. A
# file with no source is pruned here; never add its name to `expected` to keep
# a hand-maintained command — give it a source instead.
for existing in "$DEST"/*.md; do
  name="$(basename "$existing")"
  keep=0
  for e in "${expected[@]}"; do
    if [ "$name" = "$e" ]; then keep=1; break; fi
  done
  if [ "$keep" -eq 0 ]; then
    rm -- "$existing"
    echo "Removed obsolete: $name"
  fi
done

# Prune pi prompt templates whose `{project}-{name}` is no longer expected.
# Scoped to the `{project}-*.md` namespace: foreign templates are never touched.
for existing in "$PI_DEST"/"$PROJECT"-*.md; do
  [ -f "$existing" ] || continue
  name="$(basename "$existing")"
  keep=0
  for e in "${pi_expected[@]}"; do
    if [ "$name" = "$e" ]; then keep=1; break; fi
  done
  if [ "$keep" -eq 0 ]; then
    rm -- "$existing"
    echo "Removed obsolete (pi): $name"
  fi
done

# Prune reference copies whose source is gone.
for existing in "$REF_DEST"/*.md; do
  [ -f "$existing" ] || continue
  name="$(basename "$existing")"
  keep=0
  for e in ${ref_expected[@]+"${ref_expected[@]}"}; do
    if [ "$name" = "$e" ]; then keep=1; break; fi
  done
  if [ "$keep" -eq 0 ]; then
    rm -- "$existing"
    echo "Removed obsolete (reference): $name"
  fi
done

echo "Regenerated $(ls "$DEST"/*.md | wc -l | tr -d ' ') files in $DEST/"
echo "Regenerated $(ls "$PI_DEST"/"$PROJECT"-*.md | wc -l | tr -d ' ') files in $PI_DEST/"
echo "Regenerated ${#ref_expected[@]} files in $REF_DEST/"
