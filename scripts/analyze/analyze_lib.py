"""Shared helpers for the /ductus:analyze orchestration scripts (spec 060,
scenario rule-judging-runs-from-repo-scripts).

Every script runs from the project root, calls the runtime through
DUCTUS_BIN (default runtime/target/release/ductus), and keeps its staging
files in the work directory: ANALYZE_WORK when set, else
$TMPDIR/ductus-analyze/<feature>, since the CLI reads a --*-json file only
from the system temp directory. Nothing here writes a repository file.

Exit status, for every script: 0 done, 1 a check failed or the script
refused, 2 an input could not be read or the call was malformed. A failure
is never a pass.
"""

import hashlib
import json
import os
import subprocess
import sys
import tempfile


class Refused(Exception):
    """A check failed or the script refused: exit 1."""


class Unreadable(Exception):
    """An input could not be read, or the call was malformed: exit 2."""


def main(fn):
    """Run fn(argv), mapping Refused to exit 1 and Unreadable to exit 2."""
    try:
        fn(sys.argv[1:])
    except Refused as e:
        print(f"refused: {e}", file=sys.stderr)
        sys.exit(1)
    except Unreadable as e:
        print(f"unreadable: {e}", file=sys.stderr)
        sys.exit(2)


def binary():
    return os.environ.get("DUCTUS_BIN", "runtime/target/release/ductus")


def feature_dir(feature):
    d = os.path.join("specs", feature)
    if not os.path.isfile(os.path.join(d, "spec.md")):
        raise Unreadable(f"{d}/spec.md does not exist; run from the project root")
    return d


def work_dir(feature):
    w = os.environ.get("ANALYZE_WORK") or os.path.join(
        os.environ.get("TMPDIR") or tempfile.gettempdir(), "ductus-analyze", feature)
    os.makedirs(w, exist_ok=True)
    return w


def read_json(path):
    try:
        with open(path, encoding="utf-8") as f:
            return json.load(f)
    except (OSError, ValueError) as e:
        raise Unreadable(f"{path}: {e}") from e


def write_json(path, value):
    with open(path, "w", encoding="utf-8") as f:
        json.dump(value, f, ensure_ascii=False, indent=1)
        f.write("\n")


def whole(result):
    """A primitive's result with the lists its lists-file holds merged back in."""
    if isinstance(result, dict) and result.get("form") == "file" and result.get("lists-file"):
        moved = read_json(result["lists-file"])
        merged = dict(result)
        for k, v in moved.items():
            if isinstance(v, dict) and isinstance(merged.get(k), dict):
                merged[k] = {**merged[k], **v}
            else:
                merged[k] = v
        return merged
    return result


def call(*args, allow_exit=False):
    """Run one primitive and return its whole JSON result.

    A result that does not parse is Unreadable whatever the exit status. A
    non-zero exit with a parseable result is a gate the primitive reports
    (derive-dependencies exits non-zero on a cycle), returned when allow_exit
    is set and Refused otherwise.
    """
    p = subprocess.run([binary(), *args], capture_output=True, text=True)
    try:
        result = json.loads(p.stdout)
    except ValueError as e:
        raise Unreadable(f"{args[0]} exited {p.returncode} with no JSON result: "
                         f"{(p.stderr or p.stdout).strip()[:400]}") from e
    if p.returncode and not allow_exit:
        raise Refused(f"{args[0]} exited {p.returncode}: {p.stderr.strip()[:400]}")
    return whole(result), p.returncode


CORE_SUBJECTS = ("spec.md", "plan.md", "data-model.md")


def design_subjects(feature):
    """[(path relative to the feature directory, bytes)] for each design
    subject: spec, plan and data model when present, then every scenario."""
    d = feature_dir(feature)
    out = [(n, os.path.getsize(os.path.join(d, n)))
           for n in CORE_SUBJECTS if os.path.isfile(os.path.join(d, n))]
    sd = os.path.join(d, "scenarios")
    if os.path.isdir(sd):
        for n in sorted(os.listdir(sd), key=str.lower):
            if n.endswith(".md") and os.path.isfile(os.path.join(sd, n)):
                out.append((f"scenarios/{n}", os.path.getsize(os.path.join(sd, n))))
    return out


def sha256(path):
    """The file's sha256, or None when it is not a readable file."""
    try:
        with open(path, "rb") as f:
            return hashlib.sha256(f.read()).hexdigest()
    except OSError:
        return None


def is_design_subject(rel):
    if rel in CORE_SUBJECTS:
        return True
    parts = rel.split("/")
    return len(parts) == 2 and parts[0] == "scenarios" and parts[1].endswith(".md") \
        and parts[1] not in ("", ".md")


def unresolved_cites(feature, cites):
    """The cites, of the form <path>#<Heading>, that name no heading of a
    design subject. Headings are matched by read-section, whose exact-text
    match is the one write-analysis resolves a cite with."""
    d = feature_dir(feature)
    by_path, bad = {}, set()
    for c in cites:
        path, sep, heading = (c or "").partition("#")
        path, heading = path.strip(), heading.strip()
        if not sep or not heading or not is_design_subject(path) \
                or not os.path.isfile(os.path.join(d, path)):
            bad.add(c)
        else:
            by_path.setdefault(path, {}).setdefault(heading, set()).add(c)
    for path, headings in sorted(by_path.items()):
        args = ["read-section", "--path", f"{d}/{path}"]
        for h in sorted(headings):
            args += ["--heading", h]
        result, _ = call(*args)
        for h in result.get("not-found") or []:
            bad |= headings.get(h, set())
    return sorted(bad)


def manifest_path(feature):
    return os.path.join(work_dir(feature), "manifest.json")


def load_manifest(feature):
    p = manifest_path(feature)
    if not os.path.isfile(p):
        raise Unreadable(f"{p} does not exist; run scripts/analyze/split.py {feature} first")
    return read_json(p)
