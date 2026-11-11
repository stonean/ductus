---
section: "Record freshness"
---

# A-multi-number-renumber-is-one-rewrite

## Context

The mechanical-sweep exemption keeps a rename sweep from reading as a contract change ([review-staleness-on-done-specs](review-staleness-on-done-specs.md)): a file-local token pair is exempt when the sweep's repo-wide rewrites derive it. A renumber can shift several numbers in one sweep — moving `064` to `065` to make room, then `063` to `064` — and the rewrites were applied one after another, so `063` chained through `064` to `065` and a spelling-only link change read as a contract change.

## Behavior

The repo-wide rewrites apply simultaneously: one left-to-right pass, longest match first, ties broken by `(from, to)`. A sweep carrying `063` → `064` and `064` → `065` rewrites `../063-x/spec.md` to `../064-x/spec.md`, never on through to `../065-x/spec.md`. `changed_beyond_spelling` applies the same rule (audit Family 19 applied it until spec 081 retired it), and `mechanical_sweep_parity` holds it to it.

## Edge Cases

- A file whose only change since its record is such a renumber is not reported as changed.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
