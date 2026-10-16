---
section: "Follow-on scenarios"
---

# Fetch-archive-bounds-the-whole-download

## Context

`FETCH_TIMEOUT` bounds any one stall: the wait for the response, and each read
of its body
([fetch-archive-names-its-timeout](fetch-archive-names-its-timeout.md)). A body
that keeps arriving, each read returning just inside the timeout, is bounded in
size by the 256 MiB fetch cap and in nothing else. A server sending one byte
every 29 seconds holds `/ductus` on a single fetch for as long as it likes, and
the run neither fails nor says what it is waiting on.

`BE-TIMEOUT-001` asks that no call wait unboundedly. The per-read timeout
meets it one read at a time, but the call `/ductus` waits on is the whole
download.

## Behavior

`fetch-archive` declares `FETCH_DEADLINE`, 600 seconds, beside
`FETCH_TIMEOUT`, `MAX_FETCH_BYTES` and `MAX_FETCH_REDIRECTS`. Each fetch starts
its clock before the request is sent: the archive is one fetch and its checksum
sidecar another, each with its own deadline. The body is read only while the
clock is inside the deadline. A read that would begin past it fails the fetch
with an I/O error naming the deadline, and the archive is not written.

The worst case is therefore the deadline plus one `FETCH_TIMEOUT`, because a
read already in flight when the deadline passes finishes, or times out, first.
The wait for the response counts against the deadline too.

The value is set against what `/ductus` fetches. At `ductus-v0.59.7` the
largest release binary is 6,695,093 bytes and the framework archive about
3.6 MB as `git archive` packs it, so 600 seconds fails only a download slower
than about 11 KB/s on the largest file. The 256 MiB cap is a ceiling against
an unbounded body, not a size anything fetches. Reaching it inside the
deadline would take about 450 KB/s, and a body that large failing on time is
the intended outcome.

## Edge Cases

- **A download slower than the floor** now fails where it used to finish. The
  error names the deadline, so the operator sees a slow link rather than a
  hang. Retrying on a faster connection recovers, as does supplying the binary
  through `[runtime] path` (`docs/runtime.md`).
- **A test of the deadline** cannot wait ten minutes, and a test cannot serve
  a drip-fed body under TLS. The deadline is applied by a reader around the
  response body. The test drives that reader directly, with a short deadline
  over a source that never ends, so it fails if the reader ignores the
  deadline it is given.
- **Each fetch has its own deadline**, not one shared across a `fetch-archive`
  call: the sidecar is a few bytes and should not inherit what the archive
  left over.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
