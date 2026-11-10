---
section: "Follow-on scenarios"
---

# Runtime-probe-parity-audit

## Context

029 added a `command -v ductus`-equivalent binary probe to two places per agent: the bootstrap permission **seed** (the `settings_template` blob in the §Agent Registry table of `framework/bootstrap/ductus.md`, 029 Task 5) and the agent's steady-state permission set (`framework/bootstrap/configure/{key}.md`, 029 Task 6). This family first guarded that pairing. `048-govern-acquired-runtime` then replaced the probe with a filesystem check on the ductus-owned store, `~/.ductus/bin/ductus --version` (`framework/bootstrap/ductus.md` §ductus runtime detection), which the seeds of Claude, Auggie and OpenCode grant and no configure file does. The `command -v` entry stays in the seeds, but no step runs it (§Permission Setup there), so a check on it passed without testing the probe.

The probe runs on every `/ductus` run that finds no `ductus` tool, after `framework/bootstrap/ductus.md` §Permission Setup has seeded. A seed that grants the runtime CLI but drops the probe therefore prompts for it on every such run, and nothing else reports that.

## Behavior

`scripts/audit/runtime-probe-parity.sh` asserts, per agent, that an agent whose `settings_template` seed carries CLI entries also seeds the store probe, `~/.ductus/bin/ductus --version` and its `.exe` twin. The check is a fixed-string presence assertion in that agent's native grammar, scoped to the agent's own §Agent Registry row of `framework/bootstrap/ductus.md`:

- the seed carries the probe → no finding;
- the seed grants the CLI but lacks either form of the probe → a finding naming the agent and the missing literal;
- the agent's registry row is missing, or appears more than once → a finding, never a pass;
- an agent the registry lists that the family neither checks nor skips → a finding.

The configure files are not read: they grant the steady-state pipeline, which runs no probe.

The family is wired into `scripts/audit/run-all.sh` as Family 15 and enumerated in `framework/reference/audit.md`.

## Edge Cases

- **Per-agent grammar differs.** The probe is spelled `"Bash(~/.ductus/bin/ductus --version)"` and its `.exe` twin for claude, one `shellInputRegex` of `"^~/\\.ductus/bin/ductus(\\.exe)? --version$"` with an `allow` permission for auggie, and `"~/.ductus/bin/ductus --version": "allow"` and its `.exe` twin for opencode. Each literal carries its grammar's delimiters, so the Windows twin never satisfies the Unix form, and no literal is compared across grammars.
- **Row-scoped, not file-scoped.** The probe's command text also appears in the prose of `framework/bootstrap/ductus.md`, and its §MCP registration has a table keyed by the same agent names, so the family reads only its §Agent Registry table.
- **Antigravity is skipped by name on stderr.** Its seed carries no CLI entry, so every CLI call prompts there, the probe among them (`framework/bootstrap/ductus.md` §Permission Setup). A CLI entry appearing in its row (`command(.ductus/bin/ductus` or `command(~/.ductus/bin/ductus`) is a finding: the rule then applies, and the skip becomes a check.
- **Pi is skipped by name on stderr.** It has no permission-gating settings (`064-pi-host-support`), so it has no seed; Family 14 holds its `settings_template` at `{}`.
- **New agent rows.** The script names the current agents and their probe literals: an agent whose seed grants the CLI is one added `check_agent` line, and any other one `skip_agent` line with its reason. It takes the agent set from the rows of `framework/bootstrap/ductus.md` §Agent Registry, so an agent the registry lists that neither line names is a finding, and a new agent cannot pass unchecked. Read the count off the registry rather than from this bullet.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Why a scenario on 029 rather than its own feature spec?** 029 Task 9 worded the follow-up as "its own spec," but that was a sequencing decision — don't expand the audit surface while 029 was mid-implementation. With 029 done, this rides along as a scenario on the spec whose design it guards, matching the precedent set by audit Families 11–13 (each added as a scenario/subtask of the guarding spec under 022, never as a standalone feature spec). A new `specs/NNN-*/` directory for one script plus a `run_check` line plus an `audit.md` entry is heavier than the pattern warrants.
- **Why guard only the probe, not every seed entry?** The originating inbox item assumed the seed was a bootstrap subset of the configure file's full set, so every seed entry should be mirrored. Implementation (2026-06-11) found that false: the seed grants bootstrap-only commands (`tar`, `mktemp`, `git rev-parse`, `git ls-files`, the `Read(...ductus-*...)` temp globs) the steady-state configure files correctly omit, and the configure files grant pipeline commands the seed omits — neither set is a subset of the other. The only permission 029 deliberately placed in both artifacts was the binary probe, so it was the sole entry whose cross-artifact parity was a real invariant; a whole-seed check would have emitted findings against the correct repo.
- **Why does the family no longer compare the seed with the configure file?** *Resolved (owner, 2026-11-10, 029 task 15):* change the family's both-or-neither rule rather than add the store probe to every configure file. As delivered, the family held the `command -v` literal in both or neither. After 048 that literal is an entry no step runs, and the store probe is in the seeds alone, so the comparison passed without testing what detection runs. The seed is what grants the probe before it runs, so the invariant is the seed's: each agent whose seed grants the CLI seeds the probe. Rejected: seeding the probe into each `configure/{key}.md`, a grant the steady-state pipeline never uses.
