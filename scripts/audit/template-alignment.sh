#!/usr/bin/env bash
# scripts/audit/template-alignment.sh — Family 5 of /audit.
#
# Verify the spec/plan/tasks/scenario templates under framework/templates/spec/
# scaffold the fields that /{project}:analyze's blocking checks require, as
# its markdown-only reference (framework/reference/analyze.md) documents them.
# The field list is HARDCODED below; a guard keeps it from silently falling
# behind the reference.
#
# Checks (hardcoded canonical invariants, each a line-anchored grep against
# the template file; a missing template is a finding of its own):
#
#   spec.md template must scaffold:
#     - frontmatter `status:` field                  (Frontmatter schema)
#     - frontmatter `dependencies:` field            (Frontmatter schema)
#     - `## Acceptance Criteria` section             (Spec integrity)
#     - `## Open Questions` section                  (Spec integrity)
#
#   scenario.md template must scaffold:
#     - frontmatter `section:` field (post-017 schema)
#       (or `spec-ref:` for the legacy pre-017 schema; either satisfies)
#                                                    (Frontmatter schema)
#     - `## Context` section
#     - `## Behavior` section
#
#   plan.md template must scaffold:
#     - a markdown link to `spec.md`                 (Plan consistency: plan
#                                                     references the spec)
#     - `## Technical Decisions` section             (Plan consistency)
#     - `## Affected Files` section                  (Plan consistency)
#
#   tasks.md template must scaffold:
#     - a markdown link to `plan.md`                 (Task consistency: tasks
#                                                     reference the plan)
#     - at least one example numbered task (## N. ... or ### N. ...)
#                                                    (Task consistency)
#     - a `- **Done when**:` line in every example numbered task, before the
#       next heading                                 (Task consistency: each
#                                                     task has a done-when)
#
# Presence is the assertion: each check proves the template carries the
# scaffold, not that the scaffold is well-formed beyond its anchor.
#
# GUARD — the judged set. `judged_blocking_sections` below holds, one line
# each, every heading of framework/reference/analyze.md that marks a blocking
# check — a `##`..`######` heading outside fenced blocks whose parenthetical
# opens with `blocking` (including `blocking if ...`, `blocking at ...`,
# `blocking and advisory`) or reads `hard fail` — with this family's verdict
# on it: `covered` (a check above holds the template to it) or
# `not-template-related` (no template can scaffold what it checks). The
# family reports any such heading the reference has and the list lacks, and
# any listed heading the reference no longer has, so a new or renamed
# blocking check cannot land without this family deciding whether it
# scaffolds a template. An extraction that finds no such heading is a
# finding, never a pass. The guard compares heading text only; it cannot see
# a new bullet added under a heading already judged.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family template-alignment

TEMPLATE_DIR="framework/templates/spec"
ANALYZE_REF="framework/reference/analyze.md"

check_file_exists() {
  local file="$1"
  if [ ! -f "$file" ]; then
    emit "$file" "template file missing" "create $file per spec 022's template scaffolding"
    return 1
  fi
  return 0
}

check_contains() {
  local file="$1" pattern="$2" description="$3" fix="$4"
  if [ -f "$file" ] && ! grep -qE "$pattern" "$file"; then
    emit "$file" "missing $description" "$fix"
  fi
}

# The blocking headings of $ANALYZE_REF this family has judged, as
# `heading | verdict — reason`. A function rather than a heredoc inside a
# command substitution: bash 3.2 misparses the latter when its body holds
# backticks, and one heading does.
judged_blocking_sections() {
  cat <<'JUDGED'
Frontmatter schema (hard fail) | covered — spec.md status: and dependencies:, scenario.md section: (audit-record frontmatter has no template)
Spec integrity (blocking) | covered — spec.md ## Acceptance Criteria and ## Open Questions (the rest are content rules)
Artifact completeness (blocking) | not-template-related — requires a feature's plan.md, data-model.md and tasks.md to exist, which no template scaffolds
Plan consistency (blocking if plan exists) | covered — plan.md link to spec.md, ## Technical Decisions, ## Affected Files (system.md agreement is content)
Task consistency (blocking if tasks exist) | covered — tasks.md link to plan.md, a numbered example task, a Done when line on every example task
Scenario open questions (blocking at `done`, advisory otherwise) | not-template-related — counts the entries a scenario carries; an absent section reads as none
Review state drift (blocking) | not-template-related — reads review.md, which /review writes and no template scaffolds
Analyze state drift (blocking) | not-template-related — reads analysis.md, which /analyze writes and no template scaffolds
Rules (blocking and advisory) | not-template-related — validates rule files, not spec templates
JUDGED
}

# Headings in $ANALYZE_REF that mark a blocking check, one per line.
reference_blocking_sections() {
  awk '
    /^```/ { fence = !fence; next }
    fence  { next }
    /^#+ / && /\((blocking|hard fail)/ {
      sub(/^#+ +/, "")
      sub(/[[:space:]]+$/, "")
      print
    }
  ' "$ANALYZE_REF" | sort -u
}

guard_judged_set() {
  if [ ! -f "$ANALYZE_REF" ]; then
    emit "$ANALYZE_REF" "analyze reference not found — the judged set cannot be checked against it, which is not the same as clean" \
      "restore the file or update ANALYZE_REF in this script"
    return
  fi
  local reference judged
  reference="$(reference_blocking_sections)"
  judged="$(judged_blocking_sections | sed 's/ | .*//' | sort -u)"
  if [ -z "$reference" ]; then
    emit "$ANALYZE_REF" "no blocking-check headings extracted — the parse is broken, not the reference" \
      "keep the \"## Name (blocking)\" heading form, or update reference_blocking_sections in this script"
    return
  fi
  while IFS= read -r heading; do
    [ -n "$heading" ] || continue
    emit "$ANALYZE_REF" \
      "§$heading is a blocking check Family 5 has not judged — it may require a field the templates do not scaffold" \
      "decide whether a template must scaffold it: add a check above if so, then add a \"$heading | covered — ...\" or \"| not-template-related — ...\" line to judged_blocking_sections in scripts/audit/template-alignment.sh"
  done <<EOF
$(comm -23 <(printf '%s\n' "$reference") <(printf '%s\n' "$judged"))
EOF
  while IFS= read -r heading; do
    [ -n "$heading" ] || continue
    emit "scripts/audit/template-alignment.sh" \
      "judged heading \"$heading\" is no longer a blocking-check heading in $ANALYZE_REF — renamed, removed, or no longer blocking" \
      "re-judge the heading under its current name (or drop the line) in judged_blocking_sections, and revisit any check above that cites it"
  done <<EOF
$(comm -13 <(printf '%s\n' "$reference") <(printf '%s\n' "$judged"))
EOF
}

# tasks_missing_done_when FILE — print the heading of each numbered task in
# FILE with no `- **Done when**:` line before the next heading.
tasks_missing_done_when() {
  awk '
    function flush() { if (task != "" && !done) print task }
    /^#/ {
      flush()
      task = ""; done = 0
      if ($0 ~ /^(##|###) [0-9]+\./) task = $0
      next
    }
    /^- \*\*Done when\*\*:/ { done = 1 }
    END { flush() }
  ' "$1"
}

guard_judged_set

# spec.md template
if check_file_exists "$TEMPLATE_DIR/spec.md"; then
  check_contains "$TEMPLATE_DIR/spec.md" "^status:" "frontmatter status: field" "add 'status:' to the template's YAML frontmatter block"
  check_contains "$TEMPLATE_DIR/spec.md" "^dependencies:" "frontmatter dependencies: field" "add 'dependencies:' to the template's YAML frontmatter block"
  check_contains "$TEMPLATE_DIR/spec.md" "^## Acceptance Criteria" "## Acceptance Criteria section" "add the section to the template"
  check_contains "$TEMPLATE_DIR/spec.md" "^## Open Questions" "## Open Questions section" "add the section to the template"
fi

# scenario.md template
if check_file_exists "$TEMPLATE_DIR/scenario.md"; then
  if ! grep -qE "^section:|^spec-ref:" "$TEMPLATE_DIR/scenario.md"; then
    emit "$TEMPLATE_DIR/scenario.md" "missing frontmatter section: (or legacy spec-ref:) field" "add 'section:' to the template's YAML frontmatter block"
  fi
  check_contains "$TEMPLATE_DIR/scenario.md" "^## Context" "## Context section" "add the section to the template"
  check_contains "$TEMPLATE_DIR/scenario.md" "^## Behavior" "## Behavior section" "add the section to the template"
fi

# plan.md template
if check_file_exists "$TEMPLATE_DIR/plan.md"; then
  check_contains "$TEMPLATE_DIR/plan.md" '[]][(]spec[.]md[)]' "link to spec.md (analyze's Plan consistency: plan references the spec)" "add a line linking the spec, e.g. 'Implements [{NNN} — {Feature Name}](spec.md).'"
  check_contains "$TEMPLATE_DIR/plan.md" "^## Technical Decisions" "## Technical Decisions section" "add the section to the template"
  check_contains "$TEMPLATE_DIR/plan.md" "^## Affected Files" "## Affected Files section" "add the section to the template"
fi

# tasks.md template
if check_file_exists "$TEMPLATE_DIR/tasks.md"; then
  check_contains "$TEMPLATE_DIR/tasks.md" '[]][(]plan[.]md[)]' "link to plan.md (analyze's Task consistency: tasks reference the plan)" "add a line linking the plan, e.g. 'Tasks derived from the [plan](plan.md).'"
  if ! grep -qE "^(##|###) [0-9]+\." "$TEMPLATE_DIR/tasks.md"; then
    emit "$TEMPLATE_DIR/tasks.md" "no example numbered task" "add at least one ## N. (flat) or ### N. (phased) example task"
  fi
  while IFS= read -r task; do
    [ -n "$task" ] || continue
    emit "$TEMPLATE_DIR/tasks.md" \
      "example task \"$task\" has no \"- **Done when**:\" line (analyze's Task consistency: each task has a done-when)" \
      "close the example task with a '- **Done when**: ...' line, the form the tooling reads"
  done <<EOF
$(tasks_missing_done_when "$TEMPLATE_DIR/tasks.md")
EOF
fi

exit "$drift"
