---
next-task: 35
---

# 048 — Ductus-Acquired Runtime Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## Phase A — Follow-on scenarios

### 28. Record the 2026-10-29 review, analyze, and pass the completion gate

Handoff state, 2026-10-30. The review record is written (`0af1f01a`: three re-read rounds; the third round's findings routed to 051 task 35, which `ea859f7c` has since landed). `/ductus:analyze` then found 17 blocking findings against the rules as rewritten since the 2026-09-30 record; the owner decided each, and the fixes are the commit after `c8314288` whose subject names 048's analysis (spec and plan pointers, `{probe-timeout}`, the pin's grammar, a proxy value that does not parse refusing every fetch, `SSL_CERT_*` captured at startup, `--*-json` paths held to a staging directory, the hook's echoed version kept printable). That commit is `d4b75e66`, and its re-read found 1 MUST, 7 SHOULD and 10 NOTEs, recorded with recommended dispositions as task 30 (handed off by the owner, 2026-10-30, undecided). `/tmp/review-048/analysis_findings.py` drafted the analysis record's 39 findings and is scratch: re-derive them from `d4b75e66`'s message. Task 30 and task 29 are ticked, so nothing is owed beyond this review's record, `/ductus:analyze`'s re-check with `write-analysis` and its dispositions, and the completion gate verifying each criterion (AC9, AC15 and AC22 were edited); `BE-INPUT-016` stays blocking until 008 task 35 adds the forward-proxy clause.

- **Done when**: review.md records the review with no outstanding MUST or SHOULD, analysis.md is current, and `check-review-gate` passes for 048

### 32. Measure the two Windows cases no run has reached

Decided by the owner on 2026-11-13, each deferred here by 048's review. First, AC13 claims the pointer holds the resolved binary's bytes without elevated privileges, but every Windows run so far was elevated with Developer Mode on: `runtime.yml` on windows-latest, e.g. 43348658634. Second, AC10 claims State B completes in one run plus a restart, but no Windows run has had a host write State B's `--*-json` files into `{tempdir}`, a Git Bash `mktemp -d` path the CLI must find inside a staging directory. The first box runs in `runtime.yml`'s `windows-unelevated` job. The second case was to run as a real Claude Code session on `windows-latest` (owner, 2026-11-16). The owner then decided on 2026-11-17 not to run it, and its workflow was removed. `scenarios/state-b-continues-in-session.md`'s Edge Cases record that composition as unmeasured on Windows. 048's review is recorded (`fe977b6a`), but the `windows-unelevated` job and this change are code in its window, so owed in order: a review of what changed since it; `/ductus:analyze`, which `write-analysis` refuses while this task is open and which judges all 310 rules, since 048's analysis predates stored verdicts; `check-review-gate`; and the owner's approval for `done`.

- [x] Run `scripts/measure-replace-under-server.sh` on Windows as a non-admin local user with Developer Mode off: a `runtime.yml` step that turns Developer Mode off and runs the script as such a user, or a Windows machine the owner has
- [x] Record the run, its job and each pointer row's outcome in `scenarios/pointer-follows-the-store.md`'s Edge Cases and `spec.md`'s Edge Cases, in place of the sentence that calls this case unmeasured; a row that fails there is a new task

- **Done when**: every pointer row is measured on Windows unelevated with Developer Mode off, State B's json files are measured in a real agent session on Windows, and each outcome is recorded with its run, or the owner has decided the next step for a case that could not be reached

### 33. Remove runtime.yml's two Windows jobs (AC25, owner decision 2026-11-19)

- [x] `.github/workflows/runtime.yml`'s build-and-test matrix holds `ubuntu-latest` and `macos-latest` only, task 32's `windows-unelevated` job is gone, and the `.exe` conditionals and comments the Windows leg kept alive are gone or state the two images
- [x] 022's plan, and 048's spec, plan and `state-b-continues-in-session` scenario, say no CI job runs on Windows
- [x] The first runtime run after the push starts no Windows job

- **Done when**: runtime.yml starts no Windows runner, no live artifact says it does, and a pushed run shows it

### 34. Drop Windows as a supported platform: the release, the acquisition and the bootstrap (AC25, owner decision 2026-11-19)

- [x] Put to the owner first whether Windows runtime code (`cfg(windows)` branches, `.exe` handling such as `merge-status-hook`'s hook command) and the bootstrap's remaining Windows text (`.exe` twins in the seeds, `certutil`, the copy-pointer notes) go too, or stay for a `[runtime] path` user
- [x] `.github/workflows/runtime-release.yml` builds, packages and gates no Windows target, and `.github/workflows/runtime-acquisition.yml` acquires none
- [x] `framework/bootstrap/ductus.md` and its `govern.md` copy hold no Windows row in the triple table, nor any other Windows text the owner's answer above removes
- [x] `README.md` and `docs/runtime.md` list only the platforms published, AC21 and AC23 are annotated as superseded, and 022's and 067's specs state the tree (067's scenario edit reopens it)
- [x] A scratch-adopter bootstrap shows a normal run unchanged

- **Done when**: AC25 holds, every live artifact states the platforms published, and 048's review and analysis judge it
