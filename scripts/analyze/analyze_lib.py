"""Shared helpers for the /ductus:analyze orchestration scripts (spec 060,
scenario rule-judging-runs-from-repo-scripts).

Every script runs from the project root and keeps its staging files in the
work directory, since the CLI reads a --*-json file only from the system
temp directory. Nothing here writes a repository file. Three environment
variables of its own are read once, when a script starts, and listed in
docs/runtime.md's inventory, an empty value counting as unset; TMPDIR is
read beside them, as the system temp directory the CLI also reads:

- DUCTUS_BIN, the runtime binary, default DEFAULT_BIN, which must name an
  executable file;
- ANALYZE_WORK, the work directory, default $TMPDIR/WORK_DIR_NAME/<feature>,
  which must resolve inside the system temp directory;
- ANALYZE_TIMEOUT_SECONDS, how long each runtime or git call may run before
  it is stopped, default DEFAULT_TIMEOUT_SECONDS, a whole number from
  MIN_TIMEOUT_SECONDS to MAX_TIMEOUT_SECONDS.

Exit status, for every script: 0 done; 1 a check failed or the script
refused; 2 an input or setting could not be read, the call was malformed, a
call ran out of time, or the script itself failed. A failure is never a pass.
"""

import hashlib
import json
import os
import shutil
import subprocess
import sys
import tempfile
import traceback

DEFAULT_BIN = "runtime/target/release/ductus"
WORK_DIR_NAME = "ductus-analyze"
DEFAULT_TIMEOUT_SECONDS = 600
MIN_TIMEOUT_SECONDS = 1
MAX_TIMEOUT_SECONDS = 86_400
LONG_LINE = 2000
DEFAULT_SHARE_BYTES = 135_000
DEFAULT_CELL_BYTES = 480_000
MIN_BUDGET_BYTES = 1
CORE_SUBJECTS = ("spec.md", "plan.md", "data-model.md")
# framework/reference/analyze.md, Unexamined targets: the reasons excluded by
# construction. Every other reason is a target the run could not read, which
# sets process-decisions' restricted.
EXCLUDED_BY_CONSTRUCTION = frozenset({
    "not-a-live-claim", "ships-to-adopter", "root-absent", "never-existed", "gitignored",
    "status-gated", "anchor-target-unjudgeable"})

_ENV = {k: (os.environ.get(k) or None)
        for k in ("DUCTUS_BIN", "ANALYZE_WORK", "ANALYZE_TIMEOUT_SECONDS", "TMPDIR")}
_SETTINGS = {}


class Refused(Exception):
    """A check failed or the script refused: exit 1."""


class Unreadable(Exception):
    """An input or setting could not be read, or the call was malformed: exit 2."""


def _settle():
    """Validate the environment read at startup, once."""
    raw = _ENV["ANALYZE_TIMEOUT_SECONDS"]
    if raw is None:
        timeout = DEFAULT_TIMEOUT_SECONDS
    else:
        try:
            timeout = int(raw)
        except ValueError:
            timeout = 0
        if not MIN_TIMEOUT_SECONDS <= timeout <= MAX_TIMEOUT_SECONDS:
            raise Unreadable(f"ANALYZE_TIMEOUT_SECONDS is {raw!r}; it takes a whole number from "
                             f"{MIN_TIMEOUT_SECONDS} to {MAX_TIMEOUT_SECONDS}")
    temp = os.path.realpath(_ENV["TMPDIR"] or tempfile.gettempdir())
    work = None
    if _ENV["ANALYZE_WORK"]:
        work = os.path.realpath(_ENV["ANALYZE_WORK"])
        if os.path.commonpath([work, temp]) != temp:
            raise Unreadable(f"ANALYZE_WORK is {_ENV['ANALYZE_WORK']}, outside the system temp directory "
                             f"{temp}, which the CLI reads --*-json files from")
    binary = _ENV["DUCTUS_BIN"] or DEFAULT_BIN
    found = shutil.which(binary) if os.sep not in binary else binary
    if not found or not os.path.isfile(found) or not os.access(found, os.X_OK):
        named = "DUCTUS_BIN" if _ENV["DUCTUS_BIN"] else "DUCTUS_BIN's default"
        raise Unreadable(f"{named} is {binary!r}, which names no executable file; build the runtime "
                         "(cd runtime && cargo build --release) or set DUCTUS_BIN")
    _SETTINGS.update(binary=binary, timeout=timeout, temp=temp, work=work)


def main(fn):
    """Run fn(argv): Refused exits 1; Unreadable, and any other failure, 2."""
    try:
        _settle()
        fn(sys.argv[1:])
    except Refused as e:
        print(f"refused: {e}", file=sys.stderr)
        sys.exit(1)
    except Unreadable as e:
        print(f"unreadable: {e}", file=sys.stderr)
        sys.exit(2)
    except Exception:  # a defect in the script is never a failed check, and never a pass
        traceback.print_exc()
        print("the script failed; this is no result", file=sys.stderr)
        sys.exit(2)


def binary():
    return _SETTINGS["binary"]


def run_cmd(args):
    """Run a command under ANALYZE_TIMEOUT_SECONDS, returning its CompletedProcess."""
    try:
        return subprocess.run(args, capture_output=True, text=True, errors="surrogateescape",
                              timeout=_SETTINGS["timeout"])
    except subprocess.TimeoutExpired as e:
        raise Unreadable(f"{' '.join(args[:2])} ran past ANALYZE_TIMEOUT_SECONDS "
                         f"({_SETTINGS['timeout']} s) and was stopped") from e
    except OSError as e:
        raise Unreadable(f"{args[0]} could not be run: {e}") from e


def git(*args):
    p = run_cmd(["git", *args])
    return p.stdout.strip() if p.returncode == 0 else ""


def head():
    sha = git("rev-parse", "HEAD")
    if not sha:
        raise Unreadable("git rev-parse HEAD named no commit")
    return sha


def valid_feature(feature):
    """The feature's directory, specs/<feature>, refusing any name that is not
    one directory directly under specs/ holding a spec.md."""
    if not feature or feature in (".", "..") or "/" in feature or "\\" in feature \
            or feature.startswith("-"):
        raise Unreadable(f"{feature!r} is not a feature directory name")
    d = os.path.join("specs", feature)
    if not os.path.isfile(os.path.join(d, "spec.md")):
        raise Unreadable(f"{d}/spec.md does not exist; run from the project root")
    return d


feature_dir = valid_feature


def work_dir(feature):
    valid_feature(feature)
    w = _SETTINGS["work"] or os.path.join(_SETTINGS["temp"], WORK_DIR_NAME, feature)
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
    """Run one primitive and return its whole JSON result and exit status.

    A result that does not parse is Unreadable whatever the exit status. A
    non-zero exit with a parseable result is a gate the primitive reports
    (derive-dependencies exits non-zero on a cycle), returned when allow_exit
    is set and Refused otherwise.
    """
    p = run_cmd([binary(), *args])
    try:
        result = json.loads(p.stdout)
    except ValueError as e:
        raise Unreadable(f"{args[0]} exited {p.returncode} with no JSON result: "
                         f"{(p.stderr or p.stdout).strip()[:400]}") from e
    if not isinstance(result, dict):
        raise Unreadable(f"{args[0]} returned {type(result).__name__}, not an object")
    if p.returncode and not allow_exit:
        raise Refused(f"{args[0]} exited {p.returncode}: {p.stderr.strip()[:400]}")
    return whole(result), p.returncode


def design_subjects(feature):
    """[(path relative to the feature directory, bytes)] for each design
    subject: spec, plan and data model when present, then every scenario."""
    d = valid_feature(feature)
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


def detection_digest(feature, others):
    """{path: sha256} for every file under the feature directory but its two
    audit records, which a run's own record changes, and for each of others,
    the rule files and constitution detection read beside it."""
    d = valid_feature(feature)
    out = {}
    for root, dirs, files in os.walk(d):
        dirs.sort()
        for n in sorted(files):
            rel = os.path.relpath(os.path.join(root, n), d)
            if rel not in ("review.md", "analysis.md"):
                out[os.path.join(d, rel)] = sha256(os.path.join(root, n))
    for p in others:
        out[p] = sha256(p)
    return out


def tree_state():
    """A digest of the working tree's uncommitted state: the diff from HEAD
    and each untracked file git does not ignore, with its content, so an edit
    anywhere detection may read changes it."""
    h = hashlib.sha256(run_cmd(["git", "diff", "HEAD", "--binary"]).stdout.encode("utf-8", "surrogateescape"))
    others = run_cmd(["git", "ls-files", "--others", "--exclude-standard", "-z"]).stdout
    for p in sorted(x for x in others.split("\0") if x):
        h.update(p.encode("utf-8", "surrogateescape") + b"\0" + (sha256(p) or "").encode())
    return h.hexdigest()


def check_detection_current(feature, det):
    """Refuse when detect.py's results predate a commit, or an edit to the
    feature, to a rule file or constitution detection read, or anywhere else
    in the working tree."""
    meta_path = os.path.join(det, "meta.json")
    if not os.path.isfile(meta_path):
        raise Unreadable(f"{meta_path} does not exist; run scripts/analyze/detect.py {feature} first")
    meta = read_json(meta_path)
    then = meta.get("digest") or {}
    now = detection_digest(feature, [p for p in then if not p.startswith(valid_feature(feature) + os.sep)])
    changed = sorted(p for p in set(then) | set(now) if then.get(p) != now.get(p))
    if meta.get("head") != head():
        changed.insert(0, f"HEAD, {str(meta.get('head'))[:8]} then and {head()[:8]} now")
    elif meta.get("tree") != tree_state() and not changed:
        changed.append("the working tree outside the feature, its rule files and its constitution")
    if changed:
        raise Refused(f"detection predates a change ({', '.join(changed)}); "
                      f"run scripts/analyze/detect.py {feature} again")


def is_design_subject(rel):
    if rel in CORE_SUBJECTS:
        return True
    parts = rel.split("/")
    return len(parts) == 2 and parts[0] == "scenarios" and parts[1].endswith(".md") \
        and parts[1] not in ("", ".md")


def is_restricted(reasons):
    """Whether any unexamined reason is one the run could not read."""
    return any(r not in EXCLUDED_BY_CONSTRUCTION for r in reasons)


def unresolved_cites(feature, cites):
    """The cites, of the form <path>#<Heading>, that name no heading of a
    design subject. Headings are matched by read-section, whose exact-text
    match is the one write-analysis resolves a cite with."""
    d = valid_feature(feature)
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
        if not isinstance(result.get("not-found"), list):
            raise Unreadable(f"read-section returned no not-found list for {d}/{path}, "
                             "so no cite there can be said to resolve")
        for h in result["not-found"]:
            bad |= headings.get(h, set())
    return sorted(bad)


def manifest_path(feature):
    return os.path.join(work_dir(feature), "manifest.json")


def load_manifest(feature):
    p = manifest_path(feature)
    if not os.path.isfile(p):
        raise Unreadable(f"{p} does not exist; run scripts/analyze/split.py {feature} first")
    return read_json(p)
