---
spec: 015-tarball-fetch
last-run: 2026-11-09T03:09:27Z
analyzed-against: 9f92682379e04824cbc7f1999e48b609213fa8ed
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 2
analyzed-digest:
  plan.md: f6d431f3d44c5fa2b5b4a718364fdc6ff24bb8fe0e4e987fd7be300346ce1920
  review.md: 34e62074bb1e0399e8519d7e21503a035394e601a7b37bbb5be39fbce50e1ab4
  spec.md: 56650c41b70df3b0d6a6a2c054fb2f365cd3edcbf2f56c6465d7ab2df7417c08
  tasks.md: ccdfc435058eca8645979117791ce81ad8f227210c809cbba808bf6fb4daa1b8
blocking: false
dispositions:
  fixed: 7
  deferred: 0
  waived: 2
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
---

# Analysis — 015-tarball-fetch

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 2 unexamined target(s). Dispositions: 7 fixed, 0 deferred, 2 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — spec.md says Claude Code records an accepted Read outside the project root as an allow rule for that exact absolute path, with no source — `specs/015-tarball-fetch/spec.md` — **waived**: no fetched source settles it: Claude Code's permissions page, fetched 2026-11-09, documents saved rules for Bash and WebFetch approvals but not how an accepted read outside the working directory is recorded; it is the observation behind AC8's shipped globs (decided by the rule)
- grounding — spec.md says macOS and systemd-tmpfiles Linux sweep their temp directories automatically, with no source — `specs/015-tarball-fetch/spec.md` — **waived**: no fetched source settles it, and no run depends on it: an unswept temp directory costs disk, never a later run's result, since each run fetches into a fresh mktemp directory (AC13) (decided by the rule)

## Deferred to other specs

*None.*

## Unexamined targets

2 target(s) unexamined; no breakdown recorded.

## Fixed in this run

- rule-assessment — BE-TIMEOUT-001: plan.md introduces the archive fetch from codeload.github.com and names no timeout that bounds it — `specs/015-tarball-fetch/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-DATA-001: plan.md gives the archive fetch as an https URL with no TLS-version commitment — `specs/015-tarball-fetch/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-DATA-011: no 015 artifact commits to certificate and hostname validation for the archive fetch's TLS connection — `specs/015-tarball-fetch/plan.md` — **fixed** (decided by the rule)
- grounding — spec.md's Tradeoffs gives the archive as ~hundreds of KB compressed, a measured size with no source — `specs/015-tarball-fetch/spec.md` — **fixed** (decided by the rule)
- grounding — spec.md's Cleanup gives the extract as a few hundred KB, a measured size with no source — `specs/015-tarball-fetch/spec.md` — **fixed** (decided by the rule)
- grounding — plan.md gives the archive plus its extract as under 1 MB, a measured size with no source — `specs/015-tarball-fetch/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md's Trade-offs gives the temp disk the archive uses as ~1 MB, a measured size with no source — `specs/015-tarball-fetch/plan.md` — **fixed** (decided by the rule)
