#!/usr/bin/env bash
# Regenerate .claude/commands/ductus/*.md from framework/commands/*.md and
# framework/bootstrap/configure/claude.md.
#
# Substitutes {project} -> ductus and {cli-config-dir} -> .claude.
# The configure command is sourced from framework/bootstrap/configure/claude.md.
# Files in .claude/commands/ductus/ that do not correspond to a current source
# are removed so renames flow through cleanly.
#
# Flags:
#   --check    Compare generated content against the current destination;
#              exit 0 when in sync, 1 when drift exists (prints a unified
#              diff to stdout). Used by `/audit`'s check-zero precondition
#              pass to surface generator drift without writing.

set -euo pipefail

check_mode=0
for arg in "$@"; do
  case "$arg" in
    --check) check_mode=1 ;;
    -h|--help)
      sed -n '2,15p' "$0" | sed 's/^# \{0,1\}//'
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

substitute() {
  sed -e "s/{project}/$PROJECT/g" -e "s|{cli-config-dir}|$1|g"
}

# Track expected destination filenames so we can prune obsolete generated files.
expected=()

if [ "$check_mode" -eq 1 ]; then
  # --check mode: generate into a tempdir, diff against DEST, report drift
  # without modifying anything on disk.
  tmpdir="$(mktemp -d -t gen-claude-commands-check-XXXXXX)"
  trap 'rm -rf "$tmpdir"' EXIT
  for src in "$SRC"/*.md; do
    name="$(basename "$src")"
    substitute "$CONFIG_DIR" < "$src" > "$tmpdir/$name"
    expected+=("$name")
  done
  substitute "$CONFIG_DIR" < "$CONFIGURE_SRC" > "$tmpdir/configure.md"
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

  drift=0
  # Compare every expected file against DEST.
  for name in "${expected[@]}"; do
    src_path="$tmpdir/$name"
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
  exit "$drift"
fi

# Write mode (default): regenerate destination from source.
mkdir -p "$DEST"

# Generate one command per source file in framework/commands/.
for src in "$SRC"/*.md; do
  name="$(basename "$src")"
  substitute "$CONFIG_DIR" < "$src" > "$DEST/$name"
  expected+=("$name")
done

# Configure is sourced from the agent-specific permission file, named configure.md.
substitute "$CONFIG_DIR" < "$CONFIGURE_SRC" > "$DEST/configure.md"
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

# Prune any .md files in DEST that are no longer in the expected set.
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

echo "Regenerated $(ls "$DEST"/*.md | wc -l | tr -d ' ') files in $DEST/"
echo "Regenerated $(ls "$PI_DEST"/"$PROJECT"-*.md | wc -l | tr -d ' ') files in $PI_DEST/"
