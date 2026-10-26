#!/usr/bin/env bash
# scripts/audit/installer-command-parity.sh — Family 16 of /audit.
#
# The /ductus bootstrap (framework/bootstrap/ductus.md) scaffolds an
# adopter's slash commands from a HARDCODED manifest table in its
# §Per-Agent Scaffolding → Slash commands section — one
# `framework/commands/<name>.md` row per installable command. That table
# is maintained by hand and drifts silently: when a new command is added
# under framework/commands/ (spec 041's prune.md was the first to bite),
# every other integration point tends to get updated (help.md,
# configure/*.md, runtime-tools.txt) while the installer manifest is easy
# to miss. The result is a command that exists in the framework and is
# dogfooded here (gen-claude-commands.sh globs all of framework/commands/)
# yet never reaches adopters via /ductus — and, because §Slash command
# cleanup deletes any unlisted file, would be removed if hand-placed.
#
# This family pins the manifest to the source of truth: the set of
# `framework/commands/*.md` rows in ductus.md must equal the set of
# framework/commands/*.md files, minus the maintainer-only commands that
# are intentionally not shipped to adopters (listed in
# scripts/maintainer-only-commands.txt; see `excl` below).
#
# Family 2 (manifest-parity.sh) explicitly deferred the installer file
# list as out of scope; this family closes that gap for the command
# manifest specifically, anchored on the real source files rather than on
# a second prose list.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family installer-command-parity

DUCTUS="framework/bootstrap/ductus.md"
SRC_DIR="framework/commands"
SECTION="$DUCTUS §Per-Agent Scaffolding → Slash commands"

if [ ! -f "$DUCTUS" ]; then
  emit "$DUCTUS" "bootstrap file not found" "restore framework/bootstrap/ductus.md"
  exit "$drift"
fi

# Manifest command names: the `| `framework/commands/<name>.md` | ... |`
# table rows in ductus.md. Anchored on a leading `| ` + backticked source
# path so prose references to framework/commands/*.md elsewhere in the file
# (e.g. the review.disabled-rule-files pointer) are never mistaken for rows.
manifest="$(grep -oE '^\| `framework/commands/[a-z-]+\.md`' "$DUCTUS" \
  | sed -E 's/.*framework\/commands\/([a-z-]+)\.md.*/\1/' | sort -u)"

# Actual source files under framework/commands/.
actual="$(for f in "$SRC_DIR"/*.md; do basename "$f" .md; done | sort -u)"

# Maintainer-only commands intentionally excluded from the adopter manifest.
# Read from scripts/maintainer-only-commands.txt through lib.sh, so this
# family, Family 33, and gen-help-tables.sh share one list rather than three.
# An empty result is a finding, not an empty exclusion set: without the list
# a deliberately-withheld command reads as a manifest row that is missing.
excl="$(maintainer_only_commands)"
if [ -z "$excl" ]; then
  emit "scripts/maintainer-only-commands.txt" \
    "the maintainer-only command list is missing or empty, so every deliberate omission would be reported as drift" \
    "restore scripts/maintainer-only-commands.txt with one command base name per line"
  exit "$drift"
fi

# Expected manifest = actual source files minus maintainer-only exclusions.
expected="$(comm -23 <(printf '%s' "$actual") <(printf '%s' "$excl"))"

# Missing: a shippable command with no manifest row (the prune bug).
while IFS= read -r name; do
  [ -z "$name" ] && continue
  emit "$SECTION" \
    "framework/commands/$name.md has no installer manifest row — /ductus never scaffolds it for adopters" \
    "add | \`framework/commands/$name.md\` | \`{config_dir}/commands/{project}/$name.md\` | to the Slash commands table, then bump the two 'N framework/commands/*.md rows' counts"
done <<< "$(comm -23 <(printf '%s' "$expected") <(printf '%s' "$manifest"))"

# Stale: a manifest row whose source file no longer exists.
while IFS= read -r name; do
  [ -z "$name" ] && continue
  emit "$SECTION" \
    "manifest row framework/commands/$name.md has no source file under framework/commands/" \
    "remove the stale row, or restore/rename the source file"
done <<< "$(comm -23 <(printf '%s' "$manifest") <(printf '%s' "$actual"))"

# Wrongly shipped: a maintainer-only command that appears in the manifest.
while IFS= read -r name; do
  [ -z "$name" ] && continue
  emit "$SECTION" \
    "maintainer-only command framework/commands/$name.md appears in the installer manifest — it would be shipped to adopters" \
    "remove the row, or drop $name from scripts/maintainer-only-commands.txt if it is now adopter-facing"
done <<< "$(comm -12 <(printf '%s' "$manifest") <(printf '%s' "$excl"))"

# --- Reference arm (spec 071) ----------------------------------------------
#
# Each command's markdown-only reference ships as its own file, through a
# Shared Files row `framework/reference/<name>.md` -> `.ductus/reference/<name>.md`.
# The same three directions hold as for the command rows, against the same
# maintainer-only list: a command that is withheld is withheld with its
# reference. Two more are reference-specific: a row must install the file
# under its own name in `.ductus/reference/` (the command reads it by that
# path), and a reference file must belong to a command.
REF_DIR="framework/reference"
REF_SECTION="$DUCTUS §Shared Files → \`ductus\`-owned shared files"

ref_rows="$(grep -oE '^\| `framework/reference/[a-z-]+\.md` \| `[^`]*` \|' "$DUCTUS")"
ref_manifest="$(printf '%s\n' "$ref_rows" \
  | sed -nE 's/^\| `framework\/reference\/([a-z-]+)\.md`.*/\1/p' | sort -u)"
ref_actual="$(for f in "$REF_DIR"/*.md; do [ -f "$f" ] && basename "$f" .md; done | sort -u)"
# Expected rows = references whose command exists, minus maintainer-only. A
# reference with no command is reported once, as an orphan below; expecting a
# row for it too would also tell the author to add the row the orphan finding
# tells them to remove.
ref_expected="$(comm -23 <(comm -12 <(printf '%s' "$ref_actual") <(printf '%s' "$actual")) <(printf '%s' "$excl"))"

# Missing: a shippable reference with no row — its command's pointers would
# name a file /ductus never installs.
while IFS= read -r name; do
  [ -z "$name" ] && continue
  emit "$REF_SECTION" \
    "framework/reference/$name.md has no Shared Files row — /ductus never installs it, so /{project}:$name's pointers name a missing file" \
    "add | \`framework/reference/$name.md\` | \`.ductus/reference/$name.md\` | to the ductus-owned shared files table"
done <<< "$(comm -23 <(printf '%s' "$ref_expected") <(printf '%s' "$ref_manifest"))"

# Stale: a row whose source file no longer exists.
while IFS= read -r name; do
  [ -z "$name" ] && continue
  emit "$REF_SECTION" \
    "Shared Files row framework/reference/$name.md has no source file under framework/reference/" \
    "remove the stale row, or restore/rename the source file"
done <<< "$(comm -23 <(printf '%s' "$ref_manifest") <(printf '%s' "$ref_actual"))"

# Wrongly shipped: a maintainer-only command's reference in the manifest.
while IFS= read -r name; do
  [ -z "$name" ] && continue
  emit "$REF_SECTION" \
    "maintainer-only reference framework/reference/$name.md appears in the Shared Files manifest — it would be shipped to adopters" \
    "remove the row; a withheld command is withheld with its reference"
done <<< "$(comm -12 <(printf '%s' "$ref_manifest") <(printf '%s' "$excl"))"

# Wrong destination: the command reads `.ductus/reference/<name>.md`, so any
# other destination installs a file nothing reads.
while IFS= read -r row; do
  [ -z "$row" ] && continue
  name="$(printf '%s' "$row" | sed -nE 's/^\| `framework\/reference\/([a-z-]+)\.md`.*/\1/p')"
  dest="$(printf '%s' "$row" | sed -nE 's/^\| `[^`]*` \| `([^`]*)` \|$/\1/p')"
  if [ "$dest" != ".ductus/reference/$name.md" ]; then
    emit "$REF_SECTION" \
      "Shared Files row framework/reference/$name.md installs to \`$dest\` — its command reads \`.ductus/reference/$name.md\`" \
      "set the destination to \`.ductus/reference/$name.md\`"
  fi
done <<< "$ref_rows"

# Orphan: a reference file whose command does not exist.
while IFS= read -r name; do
  [ -z "$name" ] && continue
  emit "$REF_DIR/$name.md" \
    "reference file has no command — framework/commands/$name.md does not exist" \
    "remove the reference file, or restore/rename its command"
done <<< "$(comm -23 <(printf '%s' "$ref_actual") <(printf '%s' "$actual"))"

exit "$drift"
