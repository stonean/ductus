#!/usr/bin/env bash
# scripts/audit/cross-doc-consistency.sh — Family 1 of /audit.
#
# Two sub-checks for cross-doc claim consistency. Both work from a HARDCODED
# expectation held in this script — the five lifecycle states (`STATES`) and
# the back-edge transitions (`BACK_EDGES`) — and each opens with a guard that
# derives the same fact from framework/constitution.md §spec-lifecycle, the
# canonical source, and reports when the two differ. The guard is what keeps
# the hardcoded copy honest: without it, a sixth state or a new back-edge in
# the constitution would leave this family comparing the documents against a
# set that no longer matches the lifecycle, and exiting 0.
#
#   1b. Pipeline-diagram status sequence.
#
#       Guard: read the fenced `text` diagram inside the constitution's
#       §spec-lifecycle section (from its `<!-- §spec-lifecycle -->` marker to
#       the next `<!-- §` marker), take its `▶` line, and derive the state
#       sequence from the arrow structure — each state is the text before a
#       `──...──▶` edge, plus the text after the last one — so an unknown
#       state name is seen rather than filtered out. A sequence that differs
#       from `STATES`, or a diagram that yields no states, is a finding.
#
#       Comparison: in each of framework/constitution.md, README.md, and
#       framework/templates/project/project-readme.md, find the first line
#       carrying an arrow glyph (▶ or →) and every name in `STATES`, extract
#       the order in which those names first occur on it, and report README.md
#       or project-readme.md when its order differs from the constitution's.
#       A file with no such line is a finding. Only the `STATES` names are
#       read on that line — command names and gate annotations between them
#       may differ per file (project-readme uses {project}: placeholders), and
#       an extra state on a README line is not seen here; the guard is what
#       catches the constitution growing one.
#
#   1c. Back-edge wording.
#
#       Guard: read the constitution's `- **Backward via ...**` bullets in
#       §spec-lifecycle and derive each one's transition from its
#       `` `source` / `source` → `target` `` form, as one `source → target`
#       edge per source, then reduce to the distinct set. A set that differs
#       from `BACK_EDGES`, a bullet with no such form, or no bullets at all,
#       is a finding.
#
#       Comparison: framework/commands/amend.md (which owns the back-edges per
#       spec 014) and framework/commands/target.md (which surfaces their
#       next-action implications) must each match a loose presence regex for
#       the edge into `draft` and for the edge into `in-progress`. The regexes
#       are phrasing heuristics — they accept several wordings and do not
#       check sources — so this comparison proves a reference exists, never
#       that it states the edge correctly.
#
# Each sub-check emits findings to stdout in pipe-separated format. Script
# exits 0 when no findings, 1 when any sub-check produced findings.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family cross-doc

CONSTITUTION="framework/constitution.md"

# The hardcoded expectations. The 1b extraction below is built from STATES,
# and the 1c regexes test for the two targets BACK_EDGES names; each guard
# holds its copy against the constitution.
STATES="draft clarified planned in-progress done"
BACK_EDGES="clarified → draft
planned → draft
in-progress → draft
done → in-progress"

# spec_lifecycle_section — print the constitution's §spec-lifecycle section,
# from its marker to the next `<!-- §` marker (exclusive). Empty when the
# marker is absent.
spec_lifecycle_section() {
  awk '
    /^<!-- §spec-lifecycle -->/ { insec = 1; next }
    insec && /^<!-- §/          { exit }
    insec                       { print }
  ' "$CONSTITUTION" 2> /dev/null
}

# -- 1b: Pipeline-diagram status sequence -----------------------------------

# Guard: derive the state sequence from the diagram's arrow structure.
derive_constitution_states() {
  spec_lifecycle_section | awk '
    /^```/ { fence = !fence; next }
    fence && index($0, "▶") > 0 {
      n = split($0, parts, "▶")
      for (i = 1; i <= n; i++) {
        piece = parts[i]
        cut = index(piece, "──")
        if (cut > 0) piece = substr(piece, 1, cut - 1)
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", piece)
        if (piece != "") print piece
      }
      exit
    }
  '
}

guard_1b() {
  local derived expected
  derived="$(derive_constitution_states)"
  expected="$(printf '%s\n' "$STATES" | tr ' ' '\n')"
  if [ -z "$derived" ]; then
    emit "$CONSTITUTION" \
      "no lifecycle states derived from the §spec-lifecycle diagram — 1b cannot confirm its hardcoded state list, which is not the same as confirming it" \
      "keep a fenced \`text\` diagram of \`state ──/command──▶ state\` edges inside §spec-lifecycle, or update derive_constitution_states in this script to its new shape"
    return
  fi
  if [ "$derived" != "$expected" ]; then
    emit "$CONSTITUTION" \
      "the §spec-lifecycle diagram visits $(printf '%s' "$derived" | tr '\n' ' ' | sed 's/ $//') but Family 1b hardcodes $STATES — the family is comparing documents against a lifecycle the constitution no longer has" \
      "update STATES in scripts/audit/cross-doc-consistency.sh to the constitution's sequence (and the diagrams it compares), or correct the diagram"
  fi
}

# Extract the ordered state sequence from a file's pipeline diagram. The
# diagram is identified as a line containing every name in STATES AND at
# least one arrow glyph (▶ or →) — that distinguishes the diagram itself
# from prose paragraphs that happen to list the same states. Returns the
# states in order of first occurrence, one per line.
extract_diagram_states() {
  local file="$1"
  local ordered_re alternation line
  ordered_re="$(printf '%s' "$STATES" | sed 's/ /.*/g')"
  alternation="$(printf '%s' "$STATES" | tr ' ' '|')"
  # Find a line with arrows + the canonical states, in order.
  line="$(grep -m 1 -E '[▶→]' "$file" | grep -E "$ordered_re" || true)"
  if [ -z "$line" ]; then
    # Fallback: any single line containing both an arrow glyph and every
    # state, in any order. The two-stage grep above only examines the
    # file's first arrow line. Re-attempt with a per-line scan.
    line="$(awk -v states="$STATES" '
      BEGIN { n = split(states, s, " ") }
      /[▶→]/ {
        ok = 1
        for (i = 1; i <= n; i++) if (index($0, s[i]) == 0) ok = 0
        if (ok) { print; exit }
      }
    ' "$file")"
  fi
  if [ -z "$line" ]; then
    return 1
  fi
  # Extract just the state words in the order they appear. Use grep -oE for
  # portability with macOS's BSD grep.
  printf '%s\n' "$line" \
    | grep -oE "$alternation" \
    | awk '!seen[$0]++'
}

sub_1b() {
  # Three files to compare; constitution is canonical. macOS ships bash 3.2
  # which lacks associative arrays, so use parallel scalars.
  local readme="README.md"
  local project_readme="framework/templates/project/project-readme.md"
  local fix_missing
  fix_missing="add a pipeline diagram referencing $(printf '%s' "$STATES" | sed 's/ / → /g')"

  local seq_constitution seq_readme seq_project_readme
  local missing=0
  if ! seq_constitution="$(extract_diagram_states "$CONSTITUTION")"; then
    emit "$CONSTITUTION" "pipeline diagram not found (no arrow line names every lifecycle state: $STATES)" "$fix_missing"
    missing=1
  fi
  if ! seq_readme="$(extract_diagram_states "$readme")"; then
    emit "$readme" "pipeline diagram not found (no arrow line names every lifecycle state: $STATES)" "$fix_missing"
    missing=1
  fi
  if ! seq_project_readme="$(extract_diagram_states "$project_readme")"; then
    emit "$project_readme" "pipeline diagram not found (no arrow line names every lifecycle state: $STATES)" "$fix_missing"
    missing=1
  fi
  if [ "$missing" -eq 1 ]; then
    return
  fi
  if [ "$seq_readme" != "$seq_constitution" ]; then
    emit "$readme" "pipeline diagram state sequence differs from framework/constitution.md" "reconcile the diagram to visit the same states in the same order as the constitution"
  fi
  if [ "$seq_project_readme" != "$seq_constitution" ]; then
    emit "$project_readme" "pipeline diagram state sequence differs from framework/constitution.md" "reconcile the diagram to visit the same states in the same order as the constitution"
  fi
}

# -- 1c: Back-edge wording ---------------------------------------------------

# Guard: derive the distinct back-edge transitions from the "Backward via"
# bullets. Prints `source → target` per edge; a bullet that carries no
# backticked `→` form prints `!unparsed: <bullet title>` so the caller can
# report it rather than silently drop it.
derive_constitution_back_edges() {
  spec_lifecycle_section | awk '
    /^- \*\*Backward via/ {
      arrow = index($0, "→")
      if (arrow == 0) { print "!unparsed: " substr($0, 1, 60); next }
      pre = substr($0, 1, arrow - 1)
      post = substr($0, arrow + length("→"))
      if (!match(post, /`[^`]+`/)) { print "!unparsed: " substr($0, 1, 60); next }
      target = substr(post, RSTART + 1, RLENGTH - 2)
      found = 0
      while (match(pre, /`[^`]+`/)) {
        print substr(pre, RSTART + 1, RLENGTH - 2) " → " target
        found = 1
        pre = substr(pre, RSTART + RLENGTH)
      }
      if (!found) print "!unparsed: " substr($0, 1, 60)
    }
  ' | sort -u
}

guard_1c() {
  local derived expected unparsed
  derived="$(derive_constitution_back_edges)"
  if [ -z "$derived" ]; then
    emit "$CONSTITUTION" \
      "no \"Backward via\" bullets found in §spec-lifecycle — 1c cannot confirm its hardcoded back-edges, which is not the same as confirming them" \
      "keep each back-edge as a \"- **Backward via ...**\" bullet in §spec-lifecycle stating its transition as backticked states around →, or update derive_constitution_back_edges in this script to their new shape"
    return
  fi
  unparsed="$(printf '%s\n' "$derived" | grep '^!unparsed: ' || true)"
  if [ -n "$unparsed" ]; then
    while IFS= read -r line; do
      emit "$CONSTITUTION" \
        "a \"Backward via\" bullet in §spec-lifecycle carries no \`source\` → \`target\` form: ${line#!unparsed: }" \
        "state the bullet's transition as backticked states around → , or update derive_constitution_back_edges in this script"
    done <<< "$unparsed"
    return
  fi
  expected="$(printf '%s\n' "$BACK_EDGES" | sort -u)"
  if [ "$derived" != "$expected" ]; then
    emit "$CONSTITUTION" \
      "the §spec-lifecycle back-edges are {$(printf '%s' "$derived" | tr '\n' ';' | sed 's/;$//; s/;/; /g')} but Family 1c hardcodes {$(printf '%s' "$expected" | tr '\n' ';' | sed 's/;$//; s/;/; /g')} — the family checks for back-edges the constitution no longer defines, or misses one it does" \
      "update BACK_EDGES and the 1c presence regexes in scripts/audit/cross-doc-consistency.sh to the constitution's back-edges, or correct the bullets"
  fi
}

sub_1c() {
  # The canonical back-edges per spec 014 and the constitution, as BACK_EDGES
  # holds them:
  #   * clarified/planned/in-progress → draft  (new question via /amend)
  #   * done → in-progress                      (new scenario, or a meaningful
  #                                              body edit, via /amend)
  # Both should be referenced from framework/commands/amend.md (which owns
  # the back-edges) and framework/commands/target.md (which surfaces the
  # next-action implications).
  local consumers=("framework/commands/amend.md" "framework/commands/target.md")
  for c in "${consumers[@]}"; do
    if [ ! -f "$c" ]; then
      emit "$c" "expected back-edge reference missing — file does not exist" "create the file or update the family-1c consumer list"
      continue
    fi
    # Heuristic: each consumer should reference both back-edges. We accept
    # any of several phrasings.
    if ! grep -qE 'clarified.*planned.*in-progress.*draft|→ draft|reverts.*to.*draft' "$c"; then
      emit "$c" "no reference to the clarified/planned/in-progress → draft back-edge" "add a sentence referencing the back-edge per framework/constitution.md §spec-lifecycle"
    fi
    if ! grep -qE 'done.*in-progress|→ in-progress|reopens.*to.*in-progress|reverts.*to.*in-progress' "$c"; then
      emit "$c" "no reference to the done → in-progress back-edge" "add a sentence referencing the back-edge per framework/constitution.md §spec-lifecycle"
    fi
  done
}

guard_1b
sub_1b
guard_1c
sub_1c

exit "$drift"
