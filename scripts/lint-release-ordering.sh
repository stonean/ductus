#!/usr/bin/env bash
# Verify the release pipeline attempts the irreversible half first.
#
# A release has two halves and nothing makes them atomic: the crates.io
# publish, which can be yanked but never unpublished, and the GitHub release,
# which can be deleted and recreated. `.github/workflows/runtime-release.yml`
# therefore publishes the crate BEFORE the release is created, so a failure
# leaves the recoverable half undone instead of the irreversible one. Three
# releases shipped half-released before that ordering landed — ductus-v0.29.9,
# v0.30.0, and v0.31.0 — each recovered by hand.
#
# The whole guarantee is one `needs:` edge plus the placement of one action, and
# both can be undone by an edit that looks entirely reasonable in isolation.
# Nothing fails until a release is already half out the door, and the half that
# lands is the one adopters see, so nothing downstream complains. Hence a lint:
#
#   1. `release-assets` runs after `publish`.
#   2. No job but `release-assets` uses the release-upload action — uploading to
#      a release CREATES the release object when it does not exist, so any other
#      job carrying that action can bring the release into being ahead of
#      everything gating it. That is not hypothetical: ductus-v0.28.0's first
#      attempt failed its audit, published no binaries, and still left a GitHub
#      release carrying nothing but an SBOM.
#   3. Some job after `release-assets` exercises acquisition against the
#      published assets. The gating `acquire` job runs before the release
#      exists and so reads staged artifacts; without a post-release check, the
#      constitution's §runtime-boundary acquisition invariant would be enforced
#      only by a hand-dispatched workflow. AGENTS.md records that exact shape —
#      an invariant cited by name and never once executed — as one of this
#      repo's four most expensive failures.
#
# Source of truth: specs/048-govern-acquired-runtime/scenarios/release-halves-publish-together.md
# Consumed by: .github/workflows/framework-checks.yml

set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

for arg in "$@"; do
  case "$arg" in
    -h|--help)
      sed -n '2,25p' "$0" | sed 's/^# \{0,1\}//'
      echo
      echo "Usage: $(basename "$0")"
      echo "  Exits 0 when the release job runs after the crates.io publish."
      echo "  Exits 1 when the ordering is broken (errors printed to stdout)."
      exit 0
      ;;
    *) echo "Unknown argument: $arg" >&2; exit 2 ;;
  esac
done

WORKFLOW="$ROOT/.github/workflows/runtime-release.yml"
REL="${WORKFLOW#"$ROOT"/}"
RELEASE_ACTION="softprops/action-gh-release"
ACQUISITION_WORKFLOW="runtime-acquisition.yml"

if [ ! -f "$WORKFLOW" ]; then
  echo "$REL: not found — this lint cannot examine its subject, which is not the same as clean"
  exit 1
fi

# Walk the job blocks with awk rather than a YAML parser: the framework's shell
# surface has no Python or Node dependency, and the two properties below are
# structural enough to read off the indentation. BSD awk (every macOS machine)
# has to run this verbatim, so: no gensub, no length(array), no /\s/.
#
# Emits one `<job> <needs-csv> <uses-release-action> <calls-acquisition>` line
# per job.
summary="$(
  awk '
    function flush() {
      if (job != "") print job, (needs == "" ? "-" : needs), uploads, verifies
      job = ""; needs = ""; uploads = "no"; verifies = "no"; in_needs = 0
    }
    # Job keys are only recognized inside the top-level `jobs:` mapping. `on:`
    # has two-space children too (`  push:`), and reading one of those as a job
    # would attribute the steps that follow it to the wrong owner — a quiet
    # mis-parse in a lint whose whole value is being right about the graph.
    /^[A-Za-z0-9_-]+:/ {
      flush()
      in_jobs = ($0 ~ /^jobs:[[:space:]]*$/)
      next
    }
    !in_jobs { next }
    /^  [A-Za-z0-9_-]+:[[:space:]]*$/ {
      flush()
      job = $1
      sub(/:$/, "", job)
      next
    }
    job == "" { next }
    # Inline form: `needs: publish` or `needs: [audit, build]`.
    /^    needs:[[:space:]]*[^[:space:]]/ {
      needs = $0
      sub(/^    needs:[[:space:]]*/, "", needs)
      gsub(/[][ ]/, "", needs)
      in_needs = 0
      next
    }
    # Block-sequence form: a bare `needs:` followed by `- <job>` lines. Reading
    # this as an empty needs list would report a correctly-ordered workflow as
    # broken — a false alarm trains the reader to ignore the lint, which costs
    # more than the check is worth.
    /^    needs:[[:space:]]*$/ { needs = ""; in_needs = 1; next }
    in_needs && /^      -[[:space:]]*[^[:space:]]/ {
      item = $0
      sub(/^      -[[:space:]]*/, "", item)
      gsub(/[][", ]/, "", item)
      needs = (needs == "" ? item : needs "," item)
      next
    }
    /^    [A-Za-z0-9_-]+:/ { in_needs = 0 }
    index($0, RELEASE_ACTION) > 0 { uploads = "yes" }
    index($0, ACQUISITION_WORKFLOW) > 0 { verifies = "yes" }
    END { flush() }
  ' RELEASE_ACTION="$RELEASE_ACTION" \
    ACQUISITION_WORKFLOW="$ACQUISITION_WORKFLOW" "$WORKFLOW"
)"

if [ -z "$summary" ]; then
  echo "$REL: no job blocks parsed — the enumeration is broken, not the workflow"
  exit 1
fi

errors=0

release_needs=""
found_release_job=0
found_publish_job=0
found_published_check=0
while read -r job needs uploads verifies; do
  [ -n "$job" ] || continue
  case "$job" in
    release-assets)
      found_release_job=1
      release_needs="$needs"
      ;;
    publish) found_publish_job=1 ;;
    *)
      if [ "$uploads" = "yes" ]; then
        echo "$REL: job '$job' uses $RELEASE_ACTION — only release-assets may, or the release object can be created ahead of the publish that gates it"
        errors=$((errors + 1))
      fi
      ;;
  esac
  # The post-release acquisition check must run AFTER the release exists;
  # calling the acquisition workflow earlier would fetch a URL that is not
  # there yet, which is the mistake this assertion is shaped to catch.
  case ",$needs," in
    *,release-assets,*)
      if [ "$verifies" = "yes" ]; then
        found_published_check=1
      fi
      ;;
  esac
done <<EOF
$summary
EOF

# A renamed job is a broken lint, not a passing one.
if [ "$found_release_job" -eq 0 ]; then
  echo "$REL: no 'release-assets' job — this lint asserts nothing until it is pointed at the job that creates the release"
  errors=$((errors + 1))
fi
if [ "$found_publish_job" -eq 0 ]; then
  echo "$REL: no 'publish' job — this lint asserts nothing until it is pointed at the job that publishes the crate"
  errors=$((errors + 1))
fi

# `needs:` is normalized to a bare comma-separated list above, so both the
# scalar (`needs: publish`) and sequence (`needs: [publish, x]`) forms compare
# the same way.
if [ "$found_release_job" -eq 1 ]; then
  case ",$release_needs," in
    *,publish,*) ;;
    *)
      echo "$REL: release-assets does not need 'publish' (needs: $release_needs) — the GitHub release would be created before the crate is published, which is the half-release this ordering exists to prevent"
      errors=$((errors + 1))
      ;;
  esac
fi

if [ "$found_published_check" -eq 0 ]; then
  echo "$REL: no job runs after release-assets and calls $ACQUISITION_WORKFLOW — nothing would exercise acquisition against the published assets, leaving the constitution's acquisition invariant to a hand-dispatched workflow"
  errors=$((errors + 1))
fi

if [ "$errors" -gt 0 ]; then
  exit 1
fi
exit 0
