#!/usr/bin/env bash
# Regenerate the five command-group tables in framework/commands/help.md
# from each command's frontmatter `description:`.
#
# Marker pairs:
#   <!-- generated:commands-pipeline:{start,end} -->
#   <!-- generated:commands-refine:{start,end} -->
#   <!-- generated:commands-brownfield:{start,end} -->
#   <!-- generated:commands-orient:{start,end} -->
#   <!-- generated:commands-bootstrap:{start,end} -->
#
# Pipeline group has an extra "Pipeline Gate" column (gate values are
# static pipeline facts hardcoded below). All other groups are
# (Command, Description) two-column tables.
#
# Exits non-zero if any expected marker is missing, any referenced command
# source file is absent, or any framework/commands/*.md has no row in the
# tables below (so the "in sync" report cannot be made over an unexamined
# command — see the coverage assertion).

set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
HELP="$ROOT/framework/commands/help.md"

dry_run=0
for arg in "$@"; do
  case "$arg" in
    --dry-run) dry_run=1 ;;
    -h|--help)
      sed -n '2,18p' "$0" | sed 's/^# \{0,1\}//'
      echo
      echo "Usage: $(basename "$0") [--dry-run]"
      echo "  --dry-run  Report what would change; exit 1 if help.md needs updating."
      exit 0
      ;;
    *) echo "Unknown argument: $arg" >&2; exit 2 ;;
  esac
done

# Read the `description:` frontmatter field from a markdown file.
read_description() {
  local file="$1"
  if [ ! -f "$file" ]; then
    echo "Missing source file: $file" >&2
    exit 4
  fi
  awk '
    BEGIN { fm_seen = 0; in_fm = 0 }
    /^---[[:space:]]*$/ {
      if (!fm_seen) { in_fm = 1; fm_seen = 1; next }
      if (in_fm)    { in_fm = 0; exit }
    }
    in_fm && /^description:[[:space:]]/ {
      sub(/^description:[[:space:]]*/, "", $0)
      # Strip surrounding quotes if present
      gsub(/^"|"$/, "", $0)
      print $0
      exit
    }
  ' "$file"
}

# Build a two-column table: Command | Description
build_two_col_table() {
  printf '| Command | Description |\n'
  printf '| --- | --- |\n'
  while [ $# -gt 0 ]; do
    local label="$1"; shift
    local source="$1"; shift
    local desc
    desc="$(read_description "$source")"
    printf '| `%s` | %s |\n' "$label" "$desc"
  done
}

# Build the pipeline table: Command | Pipeline Gate | Description
build_pipeline_table() {
  printf '| Command | Pipeline Gate | Description |\n'
  printf '| --- | --- | --- |\n'
  while [ $# -gt 0 ]; do
    local label="$1"; shift
    local gate="$1"; shift
    local source="$1"; shift
    local desc
    desc="$(read_description "$source")"
    printf '| `%s` | %s | %s |\n' "$label" "$gate" "$desc"
  done
}

CMD_DIR="$ROOT/framework/commands"
BOOTSTRAP_DIR="$ROOT/framework/bootstrap"

# Command groups. Each entry is a (label, source) pair, or a
# (label, gate, source) triple for the pipeline group. These arrays are the
# single source for both the rendered tables and the coverage assertion
# below, so a command can never be in one and not the other.
pipeline_entries=(
  '/{project}:specify'   '→ draft'                            "$CMD_DIR/specify.md"
  '/{project}:clarify'   'draft → clarified'                  "$CMD_DIR/clarify.md"
  '/{project}:plan'      'clarified → planned'                "$CMD_DIR/plan.md"
  '/{project}:implement' 'planned → in-progress → done'       "$CMD_DIR/implement.md"
  '/{project}:review'    'blocks `done` (MUST violations)'    "$CMD_DIR/review.md"
  '/{project}:analyze'   '—'                                  "$CMD_DIR/analyze.md"
)

refine_entries=(
  '/{project}:amend'       "$CMD_DIR/amend.md"
  '/{project}:prune'       "$CMD_DIR/prune.md"
  '/{project}:fold'        "$CMD_DIR/fold.md"
  '/{project}:supersede'   "$CMD_DIR/supersede.md"
  '/{project}:consolidate' "$CMD_DIR/consolidate.md"
)

brownfield_entries=(
  '/{project}:log'   "$CMD_DIR/log.md"
  '/{project}:groom' "$CMD_DIR/groom.md"
)

orient_entries=(
  '/{project}:target' "$CMD_DIR/target.md"
  '/{project}:link'   "$CMD_DIR/link.md"
  '/{project}:status' "$CMD_DIR/status.md"
  '/{project}:help'   "$CMD_DIR/help.md"
)

bootstrap_entries=(
  '/ductus'              "$BOOTSTRAP_DIR/ductus.md"
  '/{project}:configure' "$BOOTSTRAP_DIR/configure/claude.md"
)

# Coverage assertion — the honesty half of the "in sync" claim at the end.
#
# The tables name their sources explicitly, so a command added under
# framework/commands/ but never listed above would be absent from help.md
# while this script still reported it in sync: a clean result asserted over
# a subject that was never examined. That is QUAL-CLAIM-001, and it is the
# question specs/017-derive-dont-ask/scenarios/generator-sync-claim-honesty.md
# requires this script be checked against ("can its zero count ever mean
# 'did not examine?'"). It can, so the claim is made verifiable rather than
# merely reworded: an unlisted command now fails the run.
#
# The reverse direction — a listed command whose file is gone — is already
# fatal via read_description's exit 4.
covered_commands="$(
  for entry in "${pipeline_entries[@]}" "${refine_entries[@]}" \
               "${brownfield_entries[@]}" "${orient_entries[@]}" \
               "${bootstrap_entries[@]}"; do
    # Glob match, not regex: $CMD_DIR is an absolute path whose components
    # may contain regex metacharacters. `[[ ]]` rather than `case`, whose
    # pattern-closing `)` is a parse error inside `$( )` on bash 3.2.
    if [[ "$entry" == "$CMD_DIR"/*.md ]]; then
      basename "$entry" .md
    fi
  done | sort -u
)"

actual_commands="$(for f in "$CMD_DIR"/*.md; do basename "$f" .md; done | sort -u)"

# Maintainer-only commands intentionally absent from the adopter-facing help
# tables. Read from scripts/maintainer-only-commands.txt — the single source
# shared with installer-command-parity.sh (Family 16) and
# readme-command-parity.sh (Family 33). This used to be a second copy of that
# list, carrying a comment that said so.
#
# A missing or empty list is fatal rather than "nothing is excluded": the
# coverage assertion below would then demand a help.md row for a command
# deliberately withheld from adopters, and fail every run.
MAINTAINER_ONLY="$ROOT/scripts/maintainer-only-commands.txt"
if [ ! -f "$MAINTAINER_ONLY" ]; then
  echo "gen-help-tables: $MAINTAINER_ONLY not found — cannot tell a withheld command from a missing row" >&2
  exit 6
fi
# `|| true` is load-bearing: under `set -e` with `pipefail`, a `grep` that
# matches nothing exits 1 and aborts the script here — before the guard below
# can say why. Without it the fail-closed path exits 1 silently instead of 6
# with the explanation, which is the same "a check that could not run looks
# like something else" failure the guard exists to prevent.
excluded_commands="$(sed -E 's/#.*//; s/[[:space:]]//g' "$MAINTAINER_ONLY" | grep -v '^$' | sort -u || true)"
if [ -z "$excluded_commands" ]; then
  echo "gen-help-tables: $MAINTAINER_ONLY is empty — refusing to treat every command as adopter-facing" >&2
  exit 6
fi

expected_commands="$(comm -23 <(printf '%s\n' "$actual_commands") <(printf '%s\n' "$excluded_commands"))"
uncovered="$(comm -23 <(printf '%s\n' "$expected_commands") <(printf '%s\n' "$covered_commands"))"
if [ -n "$uncovered" ]; then
  echo "gen-help-tables: command(s) under framework/commands/ with no help.md row:" >&2
  printf '  %s\n' $uncovered >&2
  echo "add each to the matching *_entries array in $(basename "$0"), or to scripts/maintainer-only-commands.txt if it is deliberately withheld from adopters" >&2
  exit 6
fi
command_count="$(printf '%s\n' "$covered_commands" | grep -c . || true)"

pipeline_table="$(build_pipeline_table "${pipeline_entries[@]}")"
refine_table="$(build_two_col_table "${refine_entries[@]}")"
brownfield_table="$(build_two_col_table "${brownfield_entries[@]}")"
orient_table="$(build_two_col_table "${orient_entries[@]}")"
bootstrap_table="$(build_two_col_table "${bootstrap_entries[@]}")"

# Splice each table between its markers. Fail if any marker is missing.
splice() {
  local marker_name="$1"
  local table_file="$2"
  local file="$3"
  if ! grep -q "<!-- generated:${marker_name}:start -->" "$file"; then
    echo "Missing marker <!-- generated:${marker_name}:start --> in $file" >&2
    return 5
  fi
  if ! grep -q "<!-- generated:${marker_name}:end -->" "$file"; then
    echo "Missing marker <!-- generated:${marker_name}:end --> in $file" >&2
    return 5
  fi
  awk -v marker="$marker_name" -v table_file="$table_file" '
    $0 ~ ("<!-- generated:" marker ":start -->") {
      print
      print ""
      while ((getline line < table_file) > 0) print line
      close(table_file)
      print ""
      in_block = 1
      next
    }
    $0 ~ ("<!-- generated:" marker ":end -->") {
      in_block = 0
      print
      next
    }
    !in_block { print }
  ' "$file"
}

tmp="$(mktemp)"
cp "$HELP" "$tmp"

# Write each table to its own temp file so awk can read multi-line content via getline.
write_table() {
  local f
  f="$(mktemp)"
  printf '%s\n' "$1" > "$f"
  echo "$f"
}

p_file="$(write_table "$pipeline_table")"
r_file="$(write_table "$refine_table")"
b_file="$(write_table "$brownfield_table")"
o_file="$(write_table "$orient_table")"
boot_file="$(write_table "$bootstrap_table")"

for pair in \
  "commands-pipeline|$p_file" \
  "commands-refine|$r_file" \
  "commands-brownfield|$b_file" \
  "commands-orient|$o_file" \
  "commands-bootstrap|$boot_file"
do
  marker="${pair%%|*}"
  table_file="${pair#*|}"
  next_tmp="$(mktemp)"
  if ! splice "$marker" "$table_file" "$tmp" > "$next_tmp"; then
    rm "$tmp" "$next_tmp" "$p_file" "$r_file" "$b_file" "$o_file" "$boot_file"
    exit 5
  fi
  mv "$next_tmp" "$tmp"
done

rm "$p_file" "$r_file" "$b_file" "$o_file" "$boot_file"

if cmp -s "$HELP" "$tmp"; then
  rm "$tmp"
  echo "No changes ($command_count command(s) in sync)"
  exit 0
fi

if [ "$dry_run" -eq 1 ]; then
  rm "$tmp"
  echo "Would update $HELP"
  exit 1
fi

mv "$tmp" "$HELP"
echo "Updated $HELP"
