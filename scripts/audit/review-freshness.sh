#!/usr/bin/env bash
# scripts/audit/review-freshness.sh — Family 19 of /audit.
#
# No `done` spec ships with a review that predates its own code.
#
# `/{project}:review` records `review.reviewed-against` in the spec
# frontmatter. Nothing compared it to reality: `check-review-gate` asserts
# `last-run` is set and `blocking` is false, and `/{project}:analyze`'s
# `review-state-drift` family asserts the same two things. Both pass for a
# review recorded against a commit whose code has since changed, so a
# passing-but-stale review is indistinguishable from a current one — and
# hand-editing a `review.md` to mark findings resolved produces exactly that
# shape.
#
# That is not hypothetical. `gvrn-v0.26.2` was tagged at `334907f` while
# spec 022's review read `reviewed-against: 1f7ee722`, three commits back;
# the adopter-scope suppression shipped unreviewed and every automated check
# passed (spec 022 review, 2026-08-03).
#
# This family is the release-time half of the same test the runtime's
# `check-review-gate` now applies at completion time. Two enforcement points
# for one rule, matching how the blocking-semantics gate is already built
# from three mutually reinforcing mechanisms rather than one.
#
# Method:
#   19a For each spec at `status: done`, read `review.reviewed-against`.
#   19b Collect the spec's **durable contracts**: `scenarios/*.md` and
#       `data-model.md`.
#   19c Emit a finding when any of them changed between `reviewed-against`
#       and HEAD.
#
# Scoped to the durable contracts, and that scoping was earned rather than
# guessed. Two wider rules were measured against this repo first:
#   * the plan's **Affected Files** — flagged 42 of 48, because old specs
#     list shared surfaces (`AGENTS.md`, `README.md`,
#     `framework/bootstrap/ductus.md`) that every later spec also touches, so
#     spec 004 read "stale" because spec 042 edited `AGENTS.md`;
#   * the spec's whole directory — flagged 31 of 48, because `tasks.md`
#     churns on every ticked checkbox and is ephemeral by construction
#     (§tasks-phase), and `plan.md` churns as Affected Files are revised.
# Both would have been disabled within a week. The durable-contract rule
# flagged 10 of 48 when it landed, caught both real failures (`gvrn-v0.26.1`
# and `gvrn-v0.26.2`, each of which added 022 scenarios after its review), and
# reported nothing on a spec whose only movement was bookkeeping. That count
# is a snapshot, not a constant: it drains as reviews are refreshed.
#
# Deliberately NOT a finding:
#   - A spec with no `review:` block. Grandfathered, matching the rule
#     `/{project}:analyze` and the shipped CI template already apply — such a
#     spec predates `/{project}:review`.
#   - A spec with no scenarios and no `data-model.md`. It has no durable
#     contract beyond `spec.md`, so there is nothing this check can compare.
#   - `tasks.md`, `plan.md`, `review.md`, and `spec.md`. The first is
#     ephemeral by construction; the rest are bookkeeping `write-review` and
#     the pipeline touch on their own. Counting any of them would make every
#     review stale the moment it was recorded.
#   - A `reviewed-against` that is not a commit in this repo (a shallow
#     clone, a rewritten history). Reported as a skip, not a finding — the
#     check cannot prove staleness it cannot resolve.
#
# Requires `git` and `python3`.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family review-freshness

for tool in git python3; do
  if ! command -v "$tool" >/dev/null 2>&1; then
    emit "(precondition)" "$tool not on PATH — cannot compare reviews against history" \
      "install $tool and re-run"
    exit 1
  fi
done

SPECS_ROOT="specs"
if [ -f "$ROOT/.ductus/config.toml" ] || [ -f "$ROOT/.govern.toml" ]; then
  cfg="$ROOT/.ductus/config.toml"; [ -f "$cfg" ] || cfg="$ROOT/.govern.toml"
  derived="$(python3 - "$cfg" <<'PY' 2>/dev/null || true
import sys, tomllib
try:
    with open(sys.argv[1], "rb") as fh:
        data = tomllib.load(fh)
except Exception:
    sys.exit(0)
value = (data.get("paths") or {}).get("specs-root")
if isinstance(value, str) and value.strip():
    print(value.strip())
PY
)"
  [ -n "$derived" ] && SPECS_ROOT="$derived"
fi

python3 - "$ROOT" "$SPECS_ROOT" <<'PY'
import hashlib
import pathlib
import re
import subprocess
import sys

root = pathlib.Path(sys.argv[1])
specs_root = sys.argv[2]
findings = []

# Coverage counters. Reported unconditionally at the end, because a family
# that prints nothing on a clean run is indistinguishable from one that
# aborted before it examined anything — `run_check` reads the exit code alone
# and cannot tell them apart either. That is `QUAL-CLAIM-001`, cited in this
# file's own header, applied to this file.
examined_digest = 0
examined_proxy = 0
grandfathered = 0
unresolvable = 0


def emit(location, message, fix):
    # Buffered rather than printed: the coverage line is computed by walking
    # the corpus, so it is only known at the end, and it has to render *above*
    # the findings it quantifies.
    findings.append(f"review-freshness | {location} | {message} | {fix}")


def frontmatter(text):
    if not text.startswith("---"):
        return ""
    end = text.find("\n---", 3)
    return text[3:end] if end != -1 else ""


def scalar(fm, key, indent=""):
    # `[ \t]*` never `\s*`: `\s` matches a newline, so a greedy run after the
    # colon walks past an empty value onto the next line and returns *that*
    # line's content as this key's — a bare `reviewed-against:` returned
    # "must-violations: 7". Verified 2026-08-28; latent, because the template
    # writes `null` rather than an empty value. The durable fix is Family 31's:
    # this parse belongs in the runtime, which deserializes frontmatter with a
    # YAML reader that cannot express the bug (§runtime-boundary).
    m = re.search(rf"^{indent}{re.escape(key)}:[ \t]*(.*)$", fm, re.M)
    if not m:
        return None
    v = m.group(1).strip().strip('"\'')
    return None if v in ("", "null", "~") else v


def is_durable_contract(rel_within_feature):
    """A scenario or the data model — the artifacts a review actually reads."""
    return (
        rel_within_feature.startswith("scenarios/")
        and rel_within_feature.endswith(".md")
    ) or rel_within_feature == "data-model.md"


def recorded_digest(fm):
    """The `review.reviewed-digest` map, or None when the record carries none.

    Three states, and the difference between the last two is the whole reason
    the runtime types this field as an `Option`:
      * absent      -> None. A pre-digest record; the caller falls back to the
                       commit-diff proxy.
      * `{}`        -> {}. Taken over a spec with no durable contracts, which
                       reads as *current*, not as unjudgeable.
      * a map       -> the recorded per-path sha256 set.
    """
    m = re.search(r"^  reviewed-digest:[ \t]*(.*)$", fm, re.M)
    if not m:
        return None
    if m.group(1).strip() == "{}":
        return {}
    digests = {}
    for line in fm[m.end():].split("\n"):
        if not line.strip():
            continue
        entry = re.match(r"^    ([^\s:]+):[ \t]*([0-9a-f]{64})[ \t]*$", line)
        if not entry:
            break  # dedent or a sibling key ends the map
        digests[entry.group(1)] = entry.group(2)
    return digests


def current_contract_digests(feature_dir):
    """(path -> sha256) for the durable contracts on disk, plus unreadable ones.

    The working tree, matching `check-review-gate` exactly. At release time the
    checkout is clean so this equals HEAD; reading the tree is what makes the
    two enforcement points answer identically rather than merely similarly.
    """
    digests = {}
    unreadable = []
    candidates = sorted(feature_dir.glob("scenarios/*.md")) + [
        feature_dir / "data-model.md"
    ]
    for path in candidates:
        if not path.is_file():
            continue
        rel = path.relative_to(feature_dir).as_posix()
        if not is_durable_contract(rel):
            continue
        try:
            digests[rel] = hashlib.sha256(path.read_bytes()).hexdigest()
        except OSError:
            # Recorded, never digested as empty: an unreadable contract is not
            # a matching one.
            unreadable.append(rel)
    return digests, unreadable


TOKEN = re.compile(r"[A-Za-z0-9_.:/-]+|\s+|.")


def pair_run(old_run, new_run):
    """Token rewrites for one removed/added line run, or None if not uniform.

    §spec-lifecycle case (a): a uniform token substitution across live
    artifacts is a *mechanical* edit — it is why a rename sweep does not
    reopen a `done` spec. A review reads contracts, and a contract that
    changed only in spelling states what it stated before, so the same rule
    has to hold here: if the sweep does not reopen the spec, it does not
    stale the review either. Without this the two rules disagree and one
    repo-wide rename turns every done spec's review stale at once — a gate
    that fires on 22 of 48 specs for a diff nobody needs to read is a gate
    people route around, the failure this family's own history records.

    Derived from the diff, never declared by the author: a commit trailer or
    an opt-out flag would make correctness depend on someone remembering to
    set it, which the AGENTS.md design principle rules out.

    Two conditions here, each of which a real edit breaks: a changed run
    replaces lines one-for-one (adding or dropping a line is structural), and
    each replaced line has the same token count. The third — that the pairs
    are ones the sweep also made in other files — is applied by
    `substitution_index`, because uniformity is a repo-wide property.

    **Substitutions may collapse.** Two old tokens may map onto one new
    token, because that is what a rename unifying two names does — 049 sent
    both `govern` and `gvrn` to `ductus`. Requiring the rewrite to be
    invertible was tried first and rejected exactly six spec-diffs in. A
    collapse that is *not* a rename (every `MUST` and every `MAY` rewritten
    to `SHOULD`) is still caught, because it fails the repo-wide test.
    """
    # A substitution replaces lines one-for-one; a run that adds or drops
    # lines is a structural edit.
    if len(old_run) != len(new_run) or not old_run:
        return None
    pairs = set()
    for old_line, new_line in zip(old_run, new_run):
        old_toks = TOKEN.findall(old_line)
        new_toks = TOKEN.findall(new_line)
        if len(old_toks) != len(new_toks):
            return None
        for a, b in zip(old_toks, new_toks):
            if a != b:
                pairs.add((a, b))
    return pairs


def diff_substitutions(base):
    """Per-file token rewrites across `base`..HEAD, from one diff.

    Maps path → set of (old, new) pairs, or None when that file's diff is not
    a pure substitution. One `git diff` per base rather than two blob reads
    per changed file: this family runs as a hard release gate, and the
    blob-per-file shape took a minute on this repo's history.
    """
    proc = subprocess.run(
        ["git", "-C", str(root), "diff", "--unified=0", f"{base}..HEAD", "--", "*.md"],
        capture_output=True,
        text=True,
        # Explicit, not the locale default: on Windows that is cp1252, which
        # raises UnicodeDecodeError on the first em-dash in a spec and takes
        # the whole family down. `replace` rather than a raise so a stray
        # undecodable byte reads as a changed token — a finding, which is the
        # safe direction — instead of crashing a release gate.
        encoding="utf-8",
        errors="replace",
    )
    result = {}
    if proc.returncode != 0:
        return result

    path = None
    old_run, new_run = [], []

    def flush():
        if not old_run and not new_run:
            return
        if path is not None and result.get(path, set()) is not None:
            pairs = pair_run(old_run, new_run)
            if pairs is None:
                result[path] = None
            else:
                result.setdefault(path, set()).update(pairs)
        old_run.clear()
        new_run.clear()

    for line in proc.stdout.split("\n"):
        if line.startswith("diff --git "):
            # Reset between files: a deleted file's header is `+++ /dev/null`,
            # which names no path. Without this its removed lines would keep
            # accumulating against the *previous* file and mark that file's
            # diff non-uniform — a pure rename reported stale because an
            # unrelated file was deleted in the same window.
            flush()
            path = None
        elif line.startswith("+++ "):
            flush()
            path = line[6:] if line.startswith("+++ b/") else None
            if path is not None:
                result.setdefault(path, set())
        elif line.startswith("@@"):
            flush()
        elif line.startswith("-") and not line.startswith("---"):
            if new_run:  # a new run started, so the previous pairing is closed
                flush()
            old_run.append(line[1:])
        elif line.startswith("+") and not line.startswith("+++"):
            new_run.append(line[1:])
    flush()
    return result


_diff_cache = {}


def substitution_index(base):
    """(per-file pairs, pairs appearing in more than one file) for `base`..HEAD.

    §spec-lifecycle case (a) calls a mechanical edit a uniform substitution
    **across live artifacts** — uniformity is a repo-wide property, not a
    file-local one, and that distinction is what keeps this check honest. A
    rename rewrites the same token in many files; a one-cell edit to a
    data-model table (`| timeout | 30s |` → `| timeout | 60s |`) rewrites one
    token in one file and reads as perfectly "uniform" on its own. Requiring
    a pair to appear in at least two files is what separates them, and a
    table-cell change is exactly the contract change this family exists to
    catch.
    """
    if base not in _diff_cache:
        per_file = diff_substitutions(base)
        counts = {}
        for pairs in per_file.values():
            if pairs:
                for pair in pairs:
                    counts[pair] = counts.get(pair, 0) + 1
        repo_wide = {p for p, n in counts.items() if n > 1}
        _diff_cache[base] = (per_file, repo_wide)
    return _diff_cache[base]


def explained_by(pair, repo_wide):
    """Whether a rewrite follows from the repo-wide ones.

    A rename produces token variants that occur in a single file — 049's
    sweep rewrote `gvrn_` to `ductus_` in exactly one data model — and those
    are consequences of the repo-wide rewrite rather than separate edits. So
    a file-local pair is admitted when applying the repo-wide substitutions
    to its old token reproduces its new token, and rejected otherwise. A
    changed table cell (`30s` → `60s`) is not derivable from any repo-wide
    rewrite, which is what keeps it a finding.
    """
    old, new = pair
    # Longest first, so a shorter rewrite cannot pre-empt a longer one.
    for x, y in sorted(repo_wide, key=lambda p: -len(p[0])):
        old = old.replace(x, y)
    return old == new


def changed_beyond_spelling(base, path):
    """Whether `path` differs from `base` by more than a repo-wide rename."""
    per_file, repo_wide = substitution_index(base)
    if path not in per_file:
        return True  # renamed, added, or deleted — a real contract change
    pairs = per_file[path]
    if pairs is None:
        return True
    if not pairs:
        return False
    # Every rewrite in this file must be one the sweep made elsewhere too,
    # or a direct consequence of one.
    return not all(
        pair in repo_wide or explained_by(pair, repo_wide) for pair in pairs
    )


specs_dir = root / specs_root
for spec_path in sorted(specs_dir.glob("*/spec.md")):
    feature = spec_path.parent.name
    rel_dir = f"{specs_root}/{feature}"
    text = spec_path.read_text(encoding="utf-8")
    fm = frontmatter(text)
    if scalar(fm, "status") != "done":
        continue
    if not re.search(r"^review:", fm, re.M):
        grandfathered += 1
        continue  # grandfathered: predates /review
    base = scalar(fm, "reviewed-against", indent="  ")
    digest = recorded_digest(fm)

    # Whether the sweep exemption can run. It needs two committed trees, so it
    # is available only when `reviewed-against` resolves — on either arm.
    base_resolves = False
    if base:
        base_resolves = subprocess.run(
            ["git", "-C", str(root), "cat-file", "-e", f"{base}^{{commit}}"],
            capture_output=True,
        ).returncode == 0

    if digest is not None:
        # --- digest arm: content, not commits --------------------------------
        # The same comparison `check-review-gate` makes, over the same field,
        # so the completion gate and this release gate cannot disagree about a
        # record that carries one.
        examined_digest += 1
        current, unreadable = current_contract_digests(spec_path.parent)
        changed = {
            rel for rel, sha in current.items() if digest.get(rel) != sha
        }
        # A contract the digest covered that is now gone is a change too.
        changed |= {rel for rel in digest if rel not in current}
        # An unreadable contract cannot be compared, so it is reported as
        # changed rather than passed over — the safe direction.
        changed |= set(unreadable)
        if base_resolves:
            changed = {
                rel
                for rel in changed
                if changed_beyond_spelling(base, f"{rel_dir}/{rel}")
            }
        stale = sorted(f"{rel_dir}/{rel}" for rel in changed)
        if stale:
            shown = ", ".join(stale[:3])
            more = f" (+{len(stale) - 3} more)" if len(stale) > 3 else ""
            emit(
                rel_dir,
                f"done spec's review is stale — {len(stale)} durable contract(s) no "
                f"longer match reviewed-digest: {shown}{more}",
                f"re-run review against {feature} before releasing",
            )
        continue

    # --- proxy arm: the commit diff, for records predating the digest --------
    # Weaker than the digest and known to be so: `write-review` stamps
    # `reviewed-against: HEAD` while reviewing the working tree, so a contract
    # reviewed before it was committed makes this diff fire on content the
    # review had already read. On this arm that false positive is guarded only
    # by the commit-then-review-then-commit convention in AGENTS.md. It is kept
    # regardless, because the alternative for a pre-digest record is examining
    # nothing at all, and every review migrates one more spec off this arm.
    if not base:
        # No digest and no sha: nothing to compare against. `not-reviewed` is
        # check-review-gate's finding, not this one — but the spec is still
        # *unexamined*, and the coverage line must not imply otherwise.
        unresolvable += 1
        continue

    if not base_resolves:
        unresolvable += 1
        emit(
            rel_dir,
            f"review names reviewed-against {base[:8]}, which is not a commit in this repo "
            "— staleness cannot be determined",
            "re-run the review so the recorded sha resolves, or restore the history "
            "(a shallow clone cannot answer this)",
        )
        continue

    examined_proxy += 1

    changed = subprocess.run(
        ["git", "-C", str(root), "diff", "--name-only", f"{base}..HEAD"],
        capture_output=True,
        text=True,
        # Explicit, not the locale default: on Windows that is cp1252, which
        # raises UnicodeDecodeError on the first em-dash in a spec and takes
        # the whole family down. `replace` rather than a raise so a stray
        # undecodable byte reads as a changed token — a finding, which is the
        # safe direction — instead of crashing a release gate.
        encoding="utf-8",
        errors="replace",
    )
    if changed.returncode != 0:
        continue

    prefix = f"{rel_dir}/"
    stale = sorted(
        p
        for p in changed.stdout.split("\n")
        if p
        and p.startswith(prefix)
        and is_durable_contract(p[len(prefix):])
        and changed_beyond_spelling(base, p)
    )
    if stale:
        shown = ", ".join(stale[:3])
        more = f" (+{len(stale) - 3} more)" if len(stale) > 3 else ""
        emit(
            rel_dir,
            f"done spec's review is stale — {len(stale)} durable contract(s) changed "
            f"since reviewed-against {base[:8]}: {shown}{more}",
            f"re-run review against {feature} before releasing",
        )

# The coverage claim, printed on every run — clean, findings, or an empty
# corpus — and *above* the findings it quantifies. The digest/proxy split is
# not decoration: the two arms do not carry the same strength of claim, and a
# bare `examined` count would assert the stronger one over both. When the proxy
# count reaches zero every record carries a digest, the arm above is dead code,
# and this line is the evidence for deleting it.
examined = examined_digest + examined_proxy
print(
    f"review-freshness: examined {examined} spec(s) at status: done — "
    f"{examined_digest} by reviewed-digest, {examined_proxy} by commit-diff proxy; "
    f"{grandfathered} grandfathered (no review: block); {unresolvable} unresolvable"
)
for finding in findings:
    print(finding)

sys.exit(1 if findings else 0)
PY
exit $?
