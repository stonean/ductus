---
spec: 061-updates-track-the-latest-release-tag
last-run: 2026-10-21T02:12:04Z
analyzed-against: db8fb327ef24ed2b3b22fc4d60e0b85d3bfa4b8e
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 9
analyzed-digest:
  data-model.md: c0392200385f4900b25aceeca1a4e38c0d9d19526e4cd7ee66030731512a7622
  plan.md: 28f5c45fc92fdae3ec523f1d869e4912215dca96e49f2bbe732ddcdc8c2d0d60
  review.md: 1859b95f806922c29497299e996bcf1f772762a83928e2c33bc813eaad9f23d9
  spec.md: 8494684bf6ba5d49275ff7a5258303407af800d301f8287f5b81b8882bbd136c
  tasks.md: 1b1f7e3ea681266ef78819b7499f4b265e1a0f288234063679f26a4e336322e1
unexamined-by-reason:
  root-absent: 9
blocking: false
dispositions:
  fixed: 8
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Analysis — 061-updates-track-the-latest-release-tag

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 9 unexamined target(s). Dispositions: 8 fixed, 0 routed, 0 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Unexamined targets

- root-absent: 9

## Fixed in this run

- rule-assessment — BE-TIMEOUT-001: plan.md names no timeout for the host-side requests Source resolution and the self-update check make to github.com and raw.githubusercontent.com (the latest-release redirect read, the pin at {raw-ref}, migrations.toml at the tag and at main, the bootstrap at {raw-ref}); only install.sh's two requests are bounded — `specs/061-updates-track-the-latest-release-tag/plan.md` — **fixed**
- rule-assessment — BE-DATA-001: plan.md commits to TLS 1.2+ only for the README one-liner; the installer's two requests and the bootstrap's host-side fetches to github.com and raw.githubusercontent.com are given as https URLs with no TLS-version commitment — `specs/061-updates-track-the-latest-release-tag/plan.md` — **fixed**
- rule-assessment — BE-DATA-011: no 061 artifact commits to certificate and hostname validation for the outbound TLS connections the installer and the bootstrap's host-side fetches open to github.com and raw.githubusercontent.com — `specs/061-updates-track-the-latest-release-tag/plan.md` — **fixed**
- rule-assessment — CFG-ENV-005: the floor halts name the tag and the violated floor but not the setting it came from, so a recorded [source] ref below a floor halts without naming [source] ref or the active config file — `specs/061-updates-track-the-latest-release-tag/spec.md` — **fixed**
- rule-assessment — CFG-CONST-001: the bootstrap's {connect-timeout} and {request-timeout} were presented as the values install.sh names, one value in two files with no check keeping them equal — `specs/061-updates-track-the-latest-release-tag/plan.md` — **fixed**
- rule-assessment — BE-DEPS-007: plan.md's CI configuration commits to no automated dependency scan — `specs/061-updates-track-the-latest-release-tag/plan.md` — **fixed**
- rule-assessment — CFG-ENV-012: 061 introduces the [source] config section without stating how its loader treats an unrecognized key — `specs/061-updates-track-the-latest-release-tag/data-model.md` — **fixed**
- rule-assessment — BE-BULK-002: plan.md's migration floor adds two registry fetches without stating the run's outcome when either fetch fails — `specs/061-updates-track-the-latest-release-tag/plan.md` — **fixed**
