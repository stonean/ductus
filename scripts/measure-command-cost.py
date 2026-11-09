#!/usr/bin/env python3
"""Measure what a typed ductus command costs in Claude Code (spec 080, AC15).

usage: python3 scripts/measure-command-cost.py [--specs N] [--hook] [--bin PATH]
                                               [--out FILE] [--keep] PROMPT ...

Builds a fixture project in a temporary directory: a generated corpus of N
specs, this repository's framework/ installed the way /ductus installs it
(apply-manifest from the archive's layout, with Claude's tool-search line),
`.mcp.json` registering the runtime, and `.ductus/bin/ductus` pointing at
the runtime binary. `--hook` also installs Claude Code's status hook with
merge-status-hook. Each PROMPT, such as "/fixture:status", then runs once
through `claude -p --output-format stream-json --verbose
--include-partial-messages` in that project, and one JSON line per run is
printed (and appended to --out) holding:

  first_output_s   seconds from launch to the first assistant text delta,
                   or to the result when no text came (a hook's answer)
  result_s         seconds from launch to the result event
  output_tokens    the result event's usage.output_tokens
  largest_tool_result_chars
                   the most characters of result text any one tool result
                   carried: its text blocks' characters, never json.dumps
                   of the content, which counts escaping (research.md,
                   Method traps)

plus what the run shows about the prompt: whether Claude Code listed the
command among its slash commands, the tools the model called in order (the
stream carries no prompt, so a call the command's body prescribes, such as
its ToolSearch line's query, is what shows the body was followed), how many
turns ran, the hook events the stream carried, and the model. A run calls the model, so this is on demand only:
no workflow references this script (spec 080). The child `claude` runs
without this shell's Claude Code session variables, with setting sources
limited to the fixture's own project and local settings, so a user's hooks
and plugins stay out of the figures.
"""

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
PROJECT = "fixture"


def run(cmd, cwd, **kw):
    return subprocess.run(cmd, cwd=cwd, check=True, capture_output=True, text=True, **kw)


def generate_corpus(root: Path, count: int) -> None:
    """`count` specs across the lifecycle, each with criteria and tasks."""
    statuses = ["draft", "clarified", "planned", "in-progress", "done"]
    for i in range(1, count + 1):
        status = statuses[i % len(statuses)]
        d = root / "specs" / f"{i:03d}-feature-{i}"
        d.mkdir(parents=True)
        criteria = "\n".join(
            f"- [{'x' if status == 'done' else ' '}] criterion {n} of feature {i}" for n in range(1, 6)
        )
        (d / "spec.md").write_text(
            f"---\nstatus: {status}\ndependencies: []\n---\n\n# {i:03d} — Feature {i}\n\n"
            f"## Behavior\n\nFeature {i} does one thing.\n\n## Acceptance Criteria\n\n{criteria}\n\n"
            "## Open Questions\n\n*None — all resolved.*\n"
        )
        if status in ("planned", "in-progress", "done"):
            (d / "plan.md").write_text(f"# {i:03d} — Feature {i} Plan\n\n## Overview\n\nBuild it.\n")
            box = "x" if status == "done" else " "
            (d / "tasks.md").write_text(
                f"# {i:03d} — Feature {i} Tasks\n\n## 1. Build it\n\n- [{box}] write it\n- [{box}] test it\n\n"
                "- **Done when**: it works.\n"
            )


def install_framework(root: Path, binary: Path) -> None:
    """Claude's command copies, as /ductus writes them: the archive's layout
    (`framework/commands/*.md` under the source root), tool-search-line set."""
    stage = Path(tempfile.mkdtemp(prefix="ductus-cost-stage-"))
    shutil.copytree(REPO / "framework", stage / "framework")
    entries = [
        {"source": f"framework/commands/{p.name}", "dest": f".claude/commands/{PROJECT}/{p.name}",
         "strategy": "update", "tool-search-line": True}
        for p in sorted((REPO / "framework/commands").glob("*.md")) if p.name != "audit.md"
    ]
    entries.append({"source": "framework/bootstrap/configure/claude.md",
                    "dest": f".claude/commands/{PROJECT}/configure.md", "strategy": "update",
                    "tool-search-line": True})
    (stage / "entries.json").write_text(json.dumps(entries))
    (stage / "subs.json").write_text(json.dumps({"project": PROJECT, "cli-config-dir": ".claude"}))
    # As /ductus runs it: in the project, the staging copy as the source root.
    run([str(binary), "apply-manifest", "--source-root", str(stage), "--target-root", ".",
         "--entries-json", str(stage / "entries.json"), "--substitutions-json", str(stage / "subs.json")],
        cwd=root)
    shutil.rmtree(stage, ignore_errors=True)


def build_fixture(specs: int, binary: Path, hook: bool) -> Path:
    root = Path(tempfile.mkdtemp(prefix="ductus-cost-"))
    run(["git", "init", "-q"], cwd=root)
    generate_corpus(root, specs)
    (root / ".ductus/bin").mkdir(parents=True)
    os.symlink(binary, root / ".ductus/bin/ductus")
    (root / ".ductus/config.toml").write_text(f'[project]\nname = "{PROJECT}"\n\n[host]\nproject = "{PROJECT}"\n')
    (root / ".mcp.json").write_text(json.dumps(
        {"mcpServers": {"ductus": {"command": ".ductus/bin/ductus", "args": ["mcp"]}}}, indent=2) + "\n")
    install_framework(root, binary)
    settings = root / ".claude/settings.local.json"
    settings.write_text(json.dumps({"permissions": {"allow": ["mcp__ductus__*"]},
                                    "enabledMcpjsonServers": ["ductus"]}, indent=2) + "\n")
    if hook:
        run([str(root / ".ductus/bin/ductus"), "merge-status-hook", "--path", ".claude/settings.local.json"],
            cwd=root, env=child_env())
    git = ["git", "-c", "user.name=fixture", "-c", "user.email=fixture@localhost"]
    run(git + ["add", "-A"], cwd=root)
    run(git + ["commit", "-qm", "fixture"], cwd=root)
    return root


def child_env() -> dict:
    """This environment less the variables that mark a Claude Code session,
    so the child is a session of its own."""
    drop = {"CLAUDECODE", "CLAUDE_CODE_SESSION_ID", "CLAUDE_CODE_ENTRYPOINT", "DUCTUS_SESSION"}
    return {k: v for k, v in os.environ.items() if k not in drop}


def text_chars(content) -> int:
    """Characters of a tool result's text: a string, or its text blocks."""
    if isinstance(content, str):
        return len(content)
    if isinstance(content, list):
        return sum(len(b.get("text", "")) for b in content if isinstance(b, dict) and b.get("type") == "text")
    return 0


def measure(root: Path, prompt: str, raw: Path | None) -> dict:
    cmd = ["claude", "-p", prompt, "--output-format", "stream-json", "--verbose",
           "--include-partial-messages", "--no-session-persistence", "--setting-sources", "project,local",
           "--mcp-config", str(root / ".mcp.json"), "--strict-mcp-config"]
    start = time.monotonic()
    # stdin closed: otherwise claude waits 3 s for piped input, inside the timings.
    proc = subprocess.Popen(cmd, cwd=root, env=child_env(), stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
                            stderr=subprocess.PIPE, text=True)
    row = {"prompt": prompt, "first_output_s": None, "result_s": None, "output_tokens": None,
           "largest_tool_result_chars": 0, "listed_slash_command": None,
           "num_turns": None, "hook_events": [], "tools_called": [], "tool_search_queries": [], "model": None, "is_error": None,
           "result_head": None}
    raw_out = raw.open("w") if raw else None
    for line in proc.stdout:
        if raw_out:
            raw_out.write(line)
        now = time.monotonic() - start
        try:
            ev = json.loads(line)
        except ValueError:
            continue
        kind, sub = ev.get("type"), ev.get("subtype")
        if kind == "system" and sub == "init":
            row["model"] = ev.get("model")
            name = prompt.split()[0].lstrip("/")
            row["listed_slash_command"] = name in (ev.get("slash_commands") or [])
        elif kind == "system" and "hook" in (sub or ""):
            row["hook_events"].append({"subtype": sub, "at_s": round(now, 3),
                                       "event": ev.get("hook_event") or ev.get("hook_event_name")})
        elif kind == "stream_event":
            delta = (ev.get("event") or {}).get("delta") or {}
            if delta.get("type") == "text_delta" and row["first_output_s"] is None:
                row["first_output_s"] = round(now, 3)
        elif kind == "assistant":
            for block in (ev.get("message") or {}).get("content") or []:
                if isinstance(block, dict) and block.get("type") == "tool_use":
                    row["tools_called"].append(block.get("name"))
                    if block.get("name") == "ToolSearch":
                        row["tool_search_queries"].append((block.get("input") or {}).get("query"))
        elif kind == "user":
            for block in (ev.get("message") or {}).get("content") or []:
                if not isinstance(block, dict):
                    continue
                if block.get("type") == "tool_result":
                    row["largest_tool_result_chars"] = max(row["largest_tool_result_chars"],
                                                           text_chars(block.get("content")))
        elif kind == "result":
            row["result_s"] = round(now, 3)
            row["output_tokens"] = (ev.get("usage") or {}).get("output_tokens")
            row["num_turns"] = ev.get("num_turns")
            row["is_error"] = ev.get("is_error")
            row["result_head"] = (ev.get("result") or "")[:160]
    proc.wait()
    if raw_out:
        raw_out.close()
    if row["first_output_s"] is None:
        row["first_output_s"] = row["result_s"]
    row["exit_code"] = proc.returncode
    row["stderr_tail"] = proc.stderr.read()[-300:] if proc.stderr else ""
    return row


def main(argv) -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("prompts", nargs="+")
    ap.add_argument("--specs", type=int, default=40)
    ap.add_argument("--hook", action="store_true", help="install the status hook in the fixture")
    ap.add_argument("--bin", type=Path, default=REPO / "runtime/target/release/ductus")
    ap.add_argument("--out", type=Path)
    ap.add_argument("--keep", action="store_true", help="leave the fixture in place")
    ap.add_argument("--raw", type=Path, help="directory to write each run's stream-json lines to")
    args = ap.parse_args(argv)
    binary = args.bin.resolve()
    if not os.access(binary, os.X_OK):
        print(f"measure-command-cost: no runtime at {binary}; build it with (cd runtime && cargo build --release)",
              file=sys.stderr)
        return 1
    if shutil.which("claude") is None:
        print("measure-command-cost: `claude` is not on PATH", file=sys.stderr)
        return 1
    root = build_fixture(args.specs, binary, args.hook)
    print(f"fixture: {root} ({args.specs} specs, hook {'installed' if args.hook else 'absent'})", file=sys.stderr)
    failed = False
    if args.raw:
        args.raw.mkdir(parents=True, exist_ok=True)
    for n, prompt in enumerate(args.prompts, 1):
        raw = args.raw / f"run-{n}.jsonl" if args.raw else None
        row = {"date": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), "specs": args.specs,
               "hook": args.hook, **measure(root, prompt, raw)}
        line = json.dumps(row)
        print(line)
        if args.out:
            with args.out.open("a") as out:
                out.write(line + "\n")
        failed |= row["result_s"] is None
    if not args.keep:
        shutil.rmtree(root, ignore_errors=True)
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
