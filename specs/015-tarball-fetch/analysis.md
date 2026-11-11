---
spec: 015-tarball-fetch
last-run: 2026-11-11T15:59:44Z
analyzed-against: a6188684c8bf1f23d1cb93c095993d1ec19420bf
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 4
analyzed-digest:
  plan.md: 48142b43f768954c99ac50ef02b4f1cde32d69bfb0c62d1234078839574ba6ca
  review.md: b1e7c14f66a4b03653f78ea5b765f4fc29755dce9a612b65c7f118dd1977cef1
  spec.md: 49f6911c79a5e86e378e34efa53f1a2e3d46c9e648fafc91068ab6b3c814d9bd
  tasks.md: acc5068bc4a693911133053fa51a5f9bfc667f5974d788a3888bbe03171b4155
unexamined-by-reason:
  root-absent: 4
blocking: false
dispositions:
  fixed: 1
  deferred: 1
  waived: 3
  undispositioned: 0
decisions:
  - key: grounding — spec.md says Claude Code records an accepted Read outside the project root as an allow rule for that exact absolute path, with no source
    outcome: waived
    reason: "no fetched source settles it: Claude Code's permissions page, fetched 2026-11-09, documents saved rules for Bash and WebFetch approvals but not how an accepted read outside the working directory is recorded; it is the observation behind AC8's shipped globs"
    decided-at: 2026-11-09T03:09:27Z
    decided-by: rule
  - key: grounding — spec.md says macOS and systemd-tmpfiles Linux sweep their temp directories automatically, with no source
    outcome: waived
    reason: "no fetched source settles it, and no run depends on it: an unswept temp directory costs disk, never a later run's result, since each run fetches into a fresh mktemp directory (AC13)"
    decided-at: 2026-11-09T03:09:27Z
    decided-by: rule
  - key: "rule-assessment — BE-INPUT-004: spec.md and plan.md extract the framework archive, and the tar -xzf fallback that runs on any extract-archive error, a refused entry included, commits to no containment of entry names"
    outcome: deferred
    target: specs/029-bootstrap-runtime-autowire/tasks.md
    task: "16"
    reason: 029 owns the per-step degradation rule the fallback applies; its task 16 has a refused entry halt the run (owner, 2026-11-11)
    decided-at: 2026-11-11T15:59:44Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — BE-TRACE-002: plan.md wraps the archive fetch, externally dependent work, in no span"
    outcome: waived
    reason: The bootstrap and the runtime's fetch-archive run in a per-agent stdio process with no tracing pipeline; adding the first span belongs to a spec about the runtime's observability, as 022's and 064's analyses decided for fetch-archive.
    decided-at: 2026-11-11T15:59:44Z
    decided-by: rule
---

# Analysis — 015-tarball-fetch

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 4 unexamined target(s). Dispositions: 1 fixed, 1 deferred, 3 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-TRACE-002: plan.md wraps the archive fetch, externally dependent work, in no span — `specs/015-tarball-fetch/plan.md` — **waived**: The bootstrap and the runtime's fetch-archive run in a per-agent stdio process with no tracing pipeline; adding the first span belongs to a spec about the runtime's observability, as 022's and 064's analyses decided for fetch-archive. (decided by the rule)
- grounding — spec.md says Claude Code records an accepted Read outside the project root as an allow rule for that exact absolute path, with no source — `specs/015-tarball-fetch/spec.md` — **waived**: no fetched source settles it: Claude Code's permissions page, fetched 2026-11-09, documents saved rules for Bash and WebFetch approvals but not how an accepted read outside the working directory is recorded; it is the observation behind AC8's shipped globs (decided by the rule)
- grounding — spec.md says macOS and systemd-tmpfiles Linux sweep their temp directories automatically, with no source — `specs/015-tarball-fetch/spec.md` — **waived**: no fetched source settles it, and no run depends on it: an unswept temp directory costs disk, never a later run's result, since each run fetches into a fresh mktemp directory (AC13) (decided by the rule)

## Deferred to other specs

- rule-assessment — BE-INPUT-004: spec.md and plan.md extract the framework archive, and the tar -xzf fallback that runs on any extract-archive error, a refused entry included, commits to no containment of entry names — `specs/015-tarball-fetch/plan.md` — **deferred** to `specs/029-bootstrap-runtime-autowire/tasks.md` task 16

## Unexamined targets

- root-absent: 4

## Fixed in this run

- rule-assessment — BE-INPUT-003: spec.md and plan.md build the curl and tar command lines from the run's ref and stated no safe-invocation mechanism — `specs/015-tarball-fetch/plan.md` — **fixed** (decided by the rule)
