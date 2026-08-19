---
section: "Acquisition"
---

# Pin-is-readable-when-acquisition-needs-it

## Context

`/ductus` could not acquire the runtime on a greenfield adoption. Every
first-time adopter is State B by definition, and State B's first act is
**Runtime acquisition**, whose step 1 read the version pin from
`{staging-dir}/ductus-main/version` and **halted** when it was absent — a
deliberate halt, because "guessing a version or falling through to 'latest'
silently installs a runtime the framework was never tested against."

That file only exists after **Archive fetch and extract**, which runs
hundreds of lines later. §Pre-flight Phase states it outright: it runs "before
Pre-run Migrations and the full archive fetch", and both its checks "run on a
small fetch or no fetch". The self-update check's small fetch pulled exactly
one file, `ductus.md`. So acquisition reached for a pin that nothing had put on
disk, and the procedure said to stop.

A second, quieter defect sat underneath it. `{tempdir}` — the only temp
directory the run creates — was created inside the **self-update check**, which
runs *after* **ductus runtime detection**. So even a fix that fetched the pin
during acquisition had nowhere to fetch it *to*. The two checks run in order
and the second one owned the resource the first one needed.

The gap was introduced with acquisition itself ([a4a3358](../spec.md)); the
pre-flight-before-archive ordering predates it. It went unnoticed because the
one recorded adopter run reached the deterministic path anyway — an agent
improvising around a halt is not the same as a procedure that works, and it is
exactly the evidence §grounding warns against accepting.

Found 2026-08-19 by an end-to-end greenfield adoption against a fresh
repository — the only test of composition this project has.

## Behavior

The pin is on disk before anything reads it, and the temp directory exists
before anything writes to it.

- **`{tempdir}` is created in the Pre-flight Phase preamble**, before either
  check, because the *first* check needs it. The self-update check no longer
  creates it and says so, so the two do not race to own it. One `mktemp` for
  the whole run, reused by the later archive fetch exactly as before.
- **Runtime acquisition step 1 fetches the pin** from
  `raw.githubusercontent.com/stonean/ductus/main/version` into
  `{tempdir}/version` and reads it there. `{pin}` in §Derived values names that
  path.
- **The halt survives, with an accurate message.** A failed fetch, or an absent
  or unparseable file, still stops the run rather than guessing a version. Only
  the reason it could fail has changed.

Pre-flight's small-fetch property is preserved. What that phase avoids is the
archive's multi-hundred-KB cost, not a `curl`; a one-line file does not
approach it.

`{staging-dir}` is retired from the procedure in the same change. It appeared
only in these acquisition steps, was never defined in §Derived values, and
named the directory the rest of the document calls `{tempdir}` — an undefined
placeholder in the one procedure a first-run adopter executes.

## Edge Cases

- **The pin and the framework tree now arrive in two fetches.** They agree
  because both name `main`. A push landing between them is the only divergence;
  it is bounded by one run, and the next `/ductus` re-acquires against the newer
  pin because acquisition is idempotent and re-probes the store. The prior
  single-archive arrangement made that divergence impossible but did so by
  reading a file that was not there, which is not a trade worth keeping.
- **The `[runtime] path` branch never reads the pin at all.** Branch 1 resolves
  a project-supplied binary and only *compares* against `{pin}` to decide
  whether to warn. A project on that branch was never blocked by this defect and
  is unaffected by the fix.
- **An offline adopter fails at the pin rather than at the asset.** The failure
  moves one step earlier and names the pin URL instead of the release URL. Both
  halt, both name what could not be reached, and the `[runtime] path` escape
  hatch is the documented answer to either.
- **`govern.md` must be updated in the same change.** Family 21 asserts the
  retired `framework/bootstrap/govern.md` stays byte-identical to `ductus.md`,
  because every pre-rename adopter's self-update fetch still resolves to that
  path. A fix landing in only one of them ships the broken procedure to exactly
  the adopters who cannot yet reach the fixed one.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
