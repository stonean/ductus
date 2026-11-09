#!/usr/bin/env python3
"""Measure what a typed ductus command costs in Claude Code (spec 080, AC15).

usage: python3 scripts/measure-command-cost.py [--specs N] [--hook] [--bin PATH]
                                               [--permissions mcp|adopter]
                                               [--target FEATURE] [--out FILE]
                                               [--raw DIR] [--keep]
                                               (PROMPT ... | --round [--only NAME,...])

Builds a fixture project in a temporary directory: a generated corpus of N
specs, then what /ductus installs for Claude Code, each file list read from
framework/bootstrap/ductus.md's own tables. That is its *ductus-owned shared
files* and *Project-specific shared files* (constitution, references, rules,
templates, project documents, hooks) and its *Slash commands* (Claude's 17
copies with the tool-search line), all through apply-manifest from the
archive's layout. Then AGENTS.md and CLAUDE.md from their templates, and
`.gitignore` holding the template's managed block, all as *Shared files with
conflict handling* writes them. Finally `.mcp.json` registering the runtime,
`.ductus/bin/ductus` pointing at the runtime binary, and the session file's
`cli-config-dir`. It installs no `/ductus` command and no managed blocks in
AGENTS.md or CLAUDE.md, and leaves the git hooks inactive.

`.claude/settings.local.json` allows `mcp__ductus__*` alone by default.
`--permissions adopter` instead holds what an adopter's file holds: the
bootstrap seed of ductus.md's `claude` registry row, as its Permission
Setup merges it, then the canonical allow and deny sets of
framework/bootstrap/configure/claude.md steps 2 and 3 through
merge-permissions, as /ductus:configure's step 1 does. Under `-p` a call
those rules do not approve is denied (Claude Code's Manual mode); Read,
Grep, Glob and read-only Bash need no rule. `--hook` also
installs Claude Code's status and help hooks with merge-status-hook. `--target`
writes the shared session default to that feature before any run, so a
command that needs a target finds one.

Each PROMPT, such as "/fixture:status", then runs once through `claude -p
--output-format stream-json --verbose --include-partial-messages` in that
project. `--round` instead runs every preset in ROUND, one per command
/ductus installs (or those `--only` names), each on a fixture of its own
with the preset's target and seed: the least state that gives the command
its main work, such as a draft spec with one open question for clarify.
One JSON line per run is printed (and appended to --out), holding:

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
turns ran, the hook events the stream carried, the result event's subtype,
the tool calls it reports denied, and its cost in US dollars, and the
model. A run calls the model, so this is on demand only:
no workflow references this script (spec 080). The child `claude` runs
without this shell's Claude Code session variables, with setting sources
limited to the fixture's own project and local settings, so a user's hooks
and plugins stay out of the figures.
"""

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
PROJECT = "fixture"
DUCTUS_MD = REPO / "framework/bootstrap/ductus.md"
CONFIGURE_MD = REPO / "framework/bootstrap/configure/claude.md"
# The bare-keyed map /ductus substitutes (ductus.md, Placeholder Substitution).
SUBSTITUTIONS = {"project": PROJECT, "cli-config-dir": ".claude", "project-name": PROJECT,
                 "One-line project description.": "A generated project for measuring command cost."}


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


def manifest_rows(heading: str) -> list[tuple[str, str]]:
    """The (source, destination) rows of the table under `heading` in
    ductus.md, the manifest /ductus applies. An empty table halts: an empty
    manifest installs nothing and would still report success."""
    text = DUCTUS_MD.read_text()
    start = text.index(f"\n{heading}\n")
    rows = []
    for line in text[start + 1:].splitlines()[1:]:
        if line.startswith("#"):
            break
        m = re.fullmatch(r"\| `([^`]+)` \| `([^`]+)` \|", line.strip())
        if m:
            rows.append((m.group(1), m.group(2)))
    if not rows:
        raise SystemExit(f"measure-command-cost: no manifest rows under {heading!r} in {DUCTUS_MD}")
    return rows


def install_framework(root: Path, binary: Path) -> None:
    """What /ductus installs for Claude Code, from its archive's layout (the
    source root holds `framework/` and `.markdownlint-cli2.jsonc`)."""
    stage = Path(tempfile.mkdtemp(prefix="ductus-cost-stage-"))
    shutil.copytree(REPO / "framework", stage / "framework")
    shutil.copy2(REPO / ".markdownlint-cli2.jsonc", stage / ".markdownlint-cli2.jsonc")
    entries = [{"source": s, "dest": d, "strategy": "update"}
               for s, d in manifest_rows("### `ductus`-owned shared files (strategy: update)")]
    entries += [{"source": s, "dest": d, "strategy": "create"}
                for s, d in manifest_rows("### Project-specific shared files (strategy: create)")]
    # Shared files with conflict handling: both `create` entries on the runtime path.
    entries += [{"source": "framework/templates/project/agents.md", "dest": "AGENTS.md", "strategy": "create"},
                {"source": "framework/templates/project/claude-md.md", "dest": "CLAUDE.md", "strategy": "create"}]
    commands = manifest_rows("### Slash commands (strategy: update)")
    if len(commands) != 17:
        raise SystemExit(f"measure-command-cost: expected 17 slash-command rows, found {len(commands)}")
    entries += [{"source": s.replace("{key}", "claude"),
                 "dest": d.replace("{config_dir}", ".claude").replace("{project}", PROJECT),
                 "strategy": "update", "tool-search-line": True} for s, d in commands]
    (stage / "entries.json").write_text(json.dumps(entries))
    (stage / "subs.json").write_text(json.dumps(SUBSTITUTIONS))
    # As /ductus runs it: in the project, the staging copy as the source root.
    run([str(binary), "apply-manifest", "--source-root", str(stage), "--target-root", ".",
         "--entries-json", str(stage / "entries.json"), "--substitutions-json", str(stage / "subs.json")],
        cwd=root)
    shutil.rmtree(stage, ignore_errors=True)
    # .gitignore, absent here: `# ductus` and the canonical block (its step 2).
    block = (REPO / "framework/templates/project/gitignore").read_text().rstrip("\n")
    (root / ".gitignore").write_text("# ductus\n" + block + "\n")


def bootstrap_seed() -> dict:
    """Claude's `settings_template`, the JSON cell of ductus.md's Agent
    Registry row for `claude`, which its Permission Setup merges first."""
    for line in DUCTUS_MD.read_text().splitlines():
        if line.startswith("| `claude` |"):
            m = re.search(r"\| `(\{.*\})` \|", line)
            if m:
                return json.loads(m.group(1))
    raise SystemExit(f"measure-command-cost: no `claude` registry row with a settings template in {DUCTUS_MD}")


def configure_bullets(step: int) -> list[str]:
    """The canonical entries of configure/claude.md's step `step` (2 allow,
    3 deny), parsed as merge-permissions' own test parses step 2."""
    text = CONFIGURE_MD.read_text()
    start = text.index(f"\n{step}. Canonical `permissions.")
    end = start + text[start:].index(f"\n{step + 1}. ")
    items = []
    for line in text[start:end].splitlines():
        item = line.strip()
        if item.startswith("- `") and item.endswith("`") and "`" not in item[3:-1]:
            items.append(item[3:-1])
    if not items:
        raise SystemExit(f"measure-command-cost: no step-{step} bullets in {CONFIGURE_MD}")
    return items


def build_fixture(specs: int, binary: Path, hook: bool, permissions: str, target: str | None,
                  seed=None) -> Path:
    root = Path(tempfile.mkdtemp(prefix="ductus-cost-"))
    run(["git", "init", "-q"], cwd=root)
    generate_corpus(root, specs)
    (root / ".ductus/bin").mkdir(parents=True)
    os.symlink(binary, root / ".ductus/bin/ductus")
    (root / ".ductus/config.toml").write_text(f'[project]\nname = "{PROJECT}"\n\n[host]\nproject = "{PROJECT}"\n')
    (root / ".mcp.json").write_text(json.dumps(
        {"mcpServers": {"ductus": {"command": ".ductus/bin/ductus", "args": ["mcp"]}}}, indent=2) + "\n")
    install_framework(root, binary)
    ductus = str(root / ".ductus/bin/ductus")
    settings = root / ".claude/settings.local.json"
    if permissions == "adopter":
        # An adopter's file: /ductus's Permission Setup merges the bootstrap
        # seed into an absent file, then /ductus:configure's step 1 merges
        # the canonical sets (merge-permissions refuses anything beyond them).
        boot = bootstrap_seed()["permissions"]
        settings.write_text(json.dumps({"permissions": {"allow": boot["allow"], "deny": boot["deny"]},
                                        "enabledMcpjsonServers": ["ductus"]}, indent=2) + "\n")
        cmd = [ductus, "merge-permissions", "--path", ".claude/settings.local.json"]
        for entry in configure_bullets(2):
            cmd.append(f"--allow={entry}")
        for entry in configure_bullets(3):
            cmd.append(f"--deny={entry}")
        run(cmd, cwd=root, env=child_env())
    else:
        settings.write_text(json.dumps({"permissions": {"allow": ["mcp__ductus__*"]},
                                        "enabledMcpjsonServers": ["ductus"]}, indent=2) + "\n")
    if hook:
        run([ductus, "merge-status-hook", "--path", ".claude/settings.local.json"], cwd=root, env=child_env())
    # The session file: /ductus's host-config write, then the target, both to
    # the shared default, which each run's identified MCP server adopts.
    run([ductus, "write-session", "--cli-config-dir", ".claude"], cwd=root, env=child_env())
    if target:
        run([ductus, "write-session", "--feature", target, "--path", f"specs/{target}"], cwd=root, env=child_env())
    commit(root, "fixture")
    if seed:
        seed(root)
    if run(["git", "status", "--porcelain"], cwd=root).stdout:
        raise SystemExit(f"measure-command-cost: the seeded fixture {root} has uncommitted changes")
    return root


GIT = ["git", "-c", "user.name=fixture", "-c", "user.email=fixture@localhost"]


def commit(root: Path, message: str) -> None:
    # A generated project: nothing in it but what this script wrote.
    run(GIT + ["add", "-A"], cwd=root)
    run(GIT + ["commit", "-qm", message], cwd=root)


# The seeds: the least each command's main path needs, committed after the
# fixture so history reads as a project's would.
GREET_PY = 'def greet(name):\n    return f"Hello, {name}!"\n'
GREET_TEST = ('import unittest\n\nfrom greet import greet\n\n\nclass GreetTest(unittest.TestCase):\n'
              '    def test_greets_by_name(self):\n        self.assertEqual(greet("Ada"), "Hello, Ada!")\n\n\n'
              'if __name__ == "__main__":\n    unittest.main()\n')
MAKEFILE = "test:\n\tpython3 -m unittest discover -s src\n"


def write_greet_feature(root: Path, feature: str, status: str, checked: bool) -> None:
    """A concrete feature in place of `feature`'s generated one: a spec, a
    plan naming its files, and a task saying what to write."""
    d, box = root / "specs" / feature, "x" if checked else " "
    (d / "spec.md").write_text(
        f"---\nstatus: {status}\ndependencies: []\n---\n\n# {feature} — Greet\n\n## Behavior\n\n"
        "`greet(name)` in `src/greet.py` returns `Hello, <name>!`.\n\n## Acceptance Criteria\n\n"
        f"- [{box}] `greet(\"Ada\")` returns `Hello, Ada!`\n"
        f"- [{box}] `make test` runs the unit tests in `src/test_greet.py`\n\n"
        "## Open Questions\n\n*None — all resolved.*\n")
    (d / "plan.md").write_text(
        f"# {feature} — Greet Plan\n\n## Overview\n\nOne function and its unit test.\n\n## Affected Files\n\n"
        "- `src/greet.py`\n- `src/test_greet.py`\n- `Makefile`\n")
    (d / "tasks.md").write_text(
        f"# {feature} — Greet Tasks\n\n## 1. Greet\n\n"
        f"- [{box}] Write `greet(name)` in `src/greet.py`, returning `Hello, <name>!`\n"
        f"- [{box}] Test it in `src/test_greet.py`, run by `make test`\n\n- **Done when**: `make test` passes.\n")


def seed_open_question(root: Path) -> None:
    """clarify: a draft spec with one open question."""
    p = root / "specs/005-feature-5/spec.md"
    p.write_text(p.read_text().replace("*None — all resolved.*",
                                       "- Should feature 5 accept an empty input, or reject it with an error?"))
    commit(root, "spec(005): one open question")


def seed_inbox_item(root: Path) -> None:
    """groom: one inbox item, through the primitive /log calls."""
    run([str(root / ".ductus/bin/ductus"), "append-inbox", "--text",
         "The status view could show how long each spec has been in its status"], cwd=root, env=child_env())
    commit(root, "log: one inbox item")


def seed_branch_spec(root: Path) -> None:
    """fold: a branch-scoped spec folding into 001."""
    d = root / "specs/cost.1-csv-export"
    d.mkdir()
    (d / "spec.md").write_text(
        "---\nstatus: draft\ndependencies: []\nfolds-into: 001-feature-1\n---\n\n# cost.1 — CSV export\n\n"
        "## Behavior\n\nFeature 1's output exports as CSV.\n\n## Acceptance Criteria\n\n"
        "- [ ] the export holds a header row and one row per record\n\n## Open Questions\n\n*None — all resolved.*\n")
    commit(root, "spec(cost.1): CSV export, folding into 001")


def seed_implement(root: Path) -> None:
    """implement: 002 (planned) with a concrete task, and a commit that
    admits `src/` and `Makefile` to derive-boundary's write boundary."""
    write_greet_feature(root, "002-feature-2", "planned", False)
    (root / "src").mkdir()
    (root / "src/.gitkeep").write_text("")
    (root / "Makefile").write_text(MAKEFILE)
    commit(root, "plan(002): greet")


def seed_review(root: Path) -> None:
    """review: a stated, verified tech stack, then 003 taken to done,
    reopened, and implemented, so its window holds code."""
    agents = root / "AGENTS.md"
    agents.write_text(re.sub(r"(## Tech Stack\n\n)<!--.*?-->",
                             r"\1| Layer | Technology | Role |\n| --- | --- | --- |\n"
                             r"| **Language** | Python 3 | Application logic |\n"
                             r"| **Tests** | unittest, run by `make test` | Unit tests |",
                             agents.read_text(), count=1, flags=re.S))
    config = root / ".ductus/config.toml"
    config.write_text(config.read_text() + "\n[review]\ntech-stack-verified = true\n")
    commit(root, "docs: tech stack")
    for name in ("spec.md", "tasks.md"):
        p = root / "specs/003-feature-3" / name
        p.write_text(p.read_text().replace("- [ ]", "- [x]").replace("status: in-progress", "status: done"))
    commit(root, "done(003)")
    write_greet_feature(root, "003-feature-3", "in-progress", False)
    commit(root, "amend(003): greet")
    write_greet_feature(root, "003-feature-3", "in-progress", True)
    (root / "src").mkdir()
    (root / "src/greet.py").write_text(GREET_PY)
    (root / "src/test_greet.py").write_text(GREET_TEST)
    (root / "Makefile").write_text(MAKEFILE)
    commit(root, "impl(003): greet")


# One preset per command /ductus installs for Claude: (prompt, session
# target written before the run, seed). Each runs on a fixture of its own.
ROUND = {
    "help": ("/fixture:help", None, None),
    "status": ("/fixture:status", None, None),
    "target": ("/fixture:target 3", None, None),
    "log": ("/fixture:log The status view could show how long each spec has been in its status", None, None),
    "specify": ("/fixture:specify Export the pipeline view as CSV for a spreadsheet", None, None),
    "clarify": ("/fixture:clarify", "005-feature-5", seed_open_question),
    "plan": ("/fixture:plan", "001-feature-1", None),
    "implement": ("/fixture:implement", "002-feature-2", seed_implement),
    "review": ("/fixture:review", "003-feature-3", seed_review),
    "analyze": ("/fixture:analyze", "004-feature-4", None),
    "amend": ("/fixture:amend What should feature 3 do when its input is empty?", "003-feature-3", None),
    "groom": ("/fixture:groom", None, seed_inbox_item),
    "prune": ("/fixture:prune", "004-feature-4", None),
    "consolidate": ("/fixture:consolidate 005-feature-5 --into 001-feature-1", None, None),
    "fold": ("/fixture:fold cost.1-csv-export", None, seed_branch_spec),
    "link": ("/fixture:link other https://github.com/example/other ../other", None, None),
    "configure": ("/fixture:configure", None, None),
}


def round_matches_manifest() -> None:
    """The round names exactly the commands ductus.md's manifest installs."""
    installed = {Path(d).stem for _, d in manifest_rows("### Slash commands (strategy: update)")}
    if installed != set(ROUND):
        raise SystemExit(f"measure-command-cost: the round's presets {sorted(set(ROUND) ^ installed)} "
                         "differ from ductus.md's slash commands")


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
           "result_subtype": None, "permission_denials": [], "total_cost_usd": None, "result_head": None}
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
            row["result_subtype"] = sub
            row["total_cost_usd"] = ev.get("total_cost_usd")
            # The authoritative record of denied calls: each names its tool
            # and input, and a Bash denial's command says which rule was missing.
            for d in ev.get("permission_denials") or []:
                inp = d.get("tool_input") or {}
                row["permission_denials"].append(
                    {"tool": d.get("tool_name"),
                     "input": (inp.get("command") or inp.get("file_path") or json.dumps(inp))[:200]})
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
    ap.add_argument("prompts", nargs="*")
    ap.add_argument("--round", action="store_true",
                    help="run each command's preset (ROUND), each on a fixture of its own, instead of PROMPTs")
    ap.add_argument("--only", help="with --round, a comma-separated list of the presets to run")
    ap.add_argument("--specs", type=int, default=40)
    ap.add_argument("--hook", action="store_true", help="install the status and help hooks in the fixture")
    ap.add_argument("--permissions", choices=["mcp", "adopter"], default="mcp",
                    help="allow mcp__ductus__* alone, or an adopter's bootstrap seed plus configure's canonical sets")
    ap.add_argument("--target", help="feature directory to write as the session target before any run")
    ap.add_argument("--bin", type=Path, default=REPO / "runtime/target/release/ductus")
    ap.add_argument("--out", type=Path)
    ap.add_argument("--keep", action="store_true", help="leave the fixture in place")
    ap.add_argument("--raw", type=Path, help="directory to write each run's stream-json lines to")
    args = ap.parse_args(argv)
    if args.round == bool(args.prompts):
        ap.error("give PROMPTs or --round, not both or neither")
    if args.round and args.target:
        ap.error("--round writes each preset's own target")
    binary = args.bin.resolve()
    if not os.access(binary, os.X_OK):
        print(f"measure-command-cost: no runtime at {binary}; build it with (cd runtime && cargo build --release)",
              file=sys.stderr)
        return 1
    if shutil.which("claude") is None:
        print("measure-command-cost: `claude` is not on PATH", file=sys.stderr)
        return 1
    if args.raw:
        args.raw.mkdir(parents=True, exist_ok=True)
    if args.round:
        round_matches_manifest()
        names = args.only.split(",") if args.only else list(ROUND)
        unknown = [n for n in names if n not in ROUND]
        if unknown:
            ap.error(f"--only names no preset: {', '.join(unknown)}")
        # (command, prompt, target, seed), one fixture each.
        runs = [(n, *ROUND[n]) for n in names]
    else:
        runs = [(None, prompt, args.target, None) for prompt in args.prompts]
    failed = False
    root = None
    for n, (command, prompt, target, seed) in enumerate(runs, 1):
        if root is None or args.round:
            root = build_fixture(args.specs, binary, args.hook, args.permissions, target, seed)
            print(f"fixture: {root} ({args.specs} specs, hooks {'installed' if args.hook else 'absent'}, "
                  f"permissions {args.permissions}, target {target or 'none'}, "
                  f"seed {seed.__name__ if seed else 'none'})", file=sys.stderr)
        raw = args.raw / f"{command or f'run-{n}'}.jsonl" if args.raw else None
        row = {"date": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), "command": command,
               "specs": args.specs, "hook": args.hook, "permissions": args.permissions, "target": target,
               "seed": seed.__name__ if seed else None, **measure(root, prompt, raw)}
        line = json.dumps(row)
        print(line, flush=True)
        if args.out:
            with args.out.open("a") as out:
                out.write(line + "\n")
        failed |= row["result_s"] is None
        if args.round and not args.keep:
            shutil.rmtree(root, ignore_errors=True)
    if not args.round and not args.keep:
        shutil.rmtree(root, ignore_errors=True)
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
