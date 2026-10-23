#!/usr/bin/env python3
"""Mutation harness: prove that each named behaviour has a test that fails
when the behaviour is removed (QUAL-TEST-001, framework/rules/quality-cross.md).

usage: python3 scripts/mutation/run.py [--check] LIST [ID ...]

LIST is a mutation list such as scripts/mutation/destination-checks.py: a
file of Python literals assigning MUTATIONS and EXPECTED_SURVIVORS, read with
ast.literal_eval and never executed. Each mutation is a tuple
(name, file, old, new, filter):

  name    "M<n> <what it removes>"; the M-number is the mutation's stable id
  file    a path under runtime/
  old     the exact text the mutation replaces, or a list of texts replaced
          together; each must occur in the file exactly once
  new     the replacement, or a list matching old
  filter  the test-name filters passed to `cargo test --lib --locked --`

EXPECTED_SURVIVORS maps an id to the reason no portable test can kill it.

Each selected mutation is written over its file, the filtered unit tests run
in the dev profile, and the file is restored, even when the run is
interrupted. A mutation is KILLED when a test fails, SURVIVED when every test
passes, and an ERROR when cargo fails another way (a compile error, say),
which is never counted as killed. runtime/src is copied before the first
mutation and compared with the copy after the last.

--check validates every selected anchor and exits without running cargo.

The exit status is 0 only when every anchor is valid, runtime/src is restored
byte for byte, no mutation errored, and the survivors are exactly the expected
ones among those selected. Do not edit anything under runtime/ while it runs:
each restore writes back the file as it was read before the mutation, so an
edit made in between is undone (AGENTS.md, Gotchas, on mutating code).
"""

import ast
import filecmp
import os
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
RUNTIME = ROOT / "runtime"


def load(list_path):
    tree = ast.parse(Path(list_path).read_text(encoding="utf-8"))
    values = {}
    for node in tree.body:
        if isinstance(node, ast.Assign) and len(node.targets) == 1 and isinstance(node.targets[0], ast.Name):
            values[node.targets[0].id] = ast.literal_eval(node.value)
    if "MUTATIONS" not in values:
        sys.exit(f"{list_path}: no MUTATIONS list")
    return values["MUTATIONS"], values.get("EXPECTED_SURVIVORS", {})


def as_list(value):
    return value if isinstance(value, list) else [value]


def anchor_counts(mutation):
    _, rel, old, _, _ = mutation
    text = (RUNTIME / rel).read_text(encoding="utf-8")
    return [text.count(o) for o in as_list(old)]


def run_one(mutation):
    name, rel, old, new, filt = mutation
    target = RUNTIME / rel
    original = target.read_text(encoding="utf-8")
    mutated = original
    for o, n in zip(as_list(old), as_list(new)):
        mutated = mutated.replace(o, n)
    started = time.monotonic()
    try:
        target.write_text(mutated, encoding="utf-8")
        result = subprocess.run(
            ["cargo", "test", "--lib", "--locked", "--", *filt.split()],
            cwd=RUNTIME,
            capture_output=True,
            text=True,
        )
    finally:
        target.write_text(original, encoding="utf-8")
    elapsed = time.monotonic() - started
    out = result.stdout + result.stderr
    failed = [
        line.split()[1].split("::")[-1]
        for line in out.splitlines()
        if line.startswith("test ") and line.endswith("FAILED")
    ]
    if failed:
        return "KILLED", f"{len(failed)} {';'.join(failed)}", elapsed
    if result.returncode == 0:
        return "SURVIVED", "", elapsed
    reason = "compile error" if "could not compile" in out or "error[" in out else f"cargo exit {result.returncode}"
    return "ERROR", reason, elapsed


def same_tree(a, b):
    cmp = filecmp.dircmp(a, b)
    if cmp.left_only or cmp.right_only or cmp.funny_files:
        return False
    _, mismatch, errors = filecmp.cmpfiles(a, b, cmp.common_files, shallow=False)
    if mismatch or errors:
        return False
    return all(same_tree(os.path.join(a, d), os.path.join(b, d)) for d in cmp.common_dirs)


def main(argv):
    check = "--check" in argv
    args = [a for a in argv if a != "--check"]
    if not args:
        sys.exit(__doc__)
    mutations, expected = load(args[0])
    only = args[1:]
    selected = [m for m in mutations if not only or m[0].split()[0] in only]
    ids = [m[0].split()[0] for m in mutations]
    duplicates = sorted({i for i in ids if ids.count(i) > 1})
    if duplicates:
        sys.exit(f"duplicate mutation ids: {' '.join(duplicates)}")

    bad = [(m[0], anchor_counts(m)) for m in selected if any(c != 1 for c in anchor_counts(m))]
    for name, counts in bad:
        print(f"{name}: ANCHOR {counts} (each anchor must occur exactly once)")
    if check:
        print(f"{len(selected)} mutation(s), {len(bad)} with an invalid anchor")
        return 1 if bad else 0

    snapshot = Path(tempfile.mkdtemp(prefix="ductus-mutation-")) / "src"
    shutil.copytree(RUNTIME / "src", snapshot)
    results = []
    try:
        for mutation in selected:
            if any(mutation[0] == b[0] for b in bad):
                continue
            verdict, detail, elapsed = run_one(mutation)
            results.append((mutation[0], verdict))
            print(f"{mutation[0]}: {verdict} {detail} ({elapsed:.0f}s)".replace("  ", " "), flush=True)
    finally:
        restored = same_tree(snapshot, RUNTIME / "src")
        print(f"runtime/src restored: {'yes' if restored else 'NO, compare it with ' + str(snapshot)}")

    survivors = {name.split()[0] for name, verdict in results if verdict == "SURVIVED"}
    errors = [name for name, verdict in results if verdict == "ERROR"]
    expected_here = {i for i in expected if i in {m[0].split()[0] for m in selected}}
    killed = sum(1 for _, verdict in results if verdict == "KILLED")
    print(f"{len(results)} run: {killed} killed, {len(survivors)} survived, {len(errors)} errored, {len(bad)} skipped for an invalid anchor")
    for i in sorted(survivors - expected_here):
        print(f"{i}: survived and is not an expected survivor")
    for i in sorted(expected_here - survivors):
        print(f"{i}: expected to survive ({expected[i]}) but did not; update the list")
    ok = restored and not bad and not errors and survivors == expected_here
    if restored:
        shutil.rmtree(snapshot.parent)
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
