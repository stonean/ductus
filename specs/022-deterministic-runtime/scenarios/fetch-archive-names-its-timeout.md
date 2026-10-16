---
section: "Follow-on scenarios"
---

# Fetch-archive-names-its-timeout

## Context

`fetch-archive` makes the runtime's only outbound HTTP request, and
`BE-TIMEOUT-001` asks that the timeout bounding it be a named constant
(`CFG-CONST-003`). The plan's `fetch-archive` transport section, written in
0.59.4's analysis, records that the request runs under `reqwest`'s
blocking-client default of 30 seconds and that `fetch-archive` does not
override it. A library default is not a constant of the runtime's own: no line
of the runtime states the bound, and a `reqwest` upgrade that changed the
default would change it silently.

Reading `reqwest` 0.13.5 to name the value also corrected what the plan said
it bounds. The plan calls it "a 30-second total timeout". It is not total. The
blocking client waits up to the timeout for the response, and the response
body's `Read` implementation applies the same timeout to each read
(`blocking/response.rs`). So it bounds any one stall. A body that keeps
arriving is bounded in size by the 256 MiB fetch cap and not in duration.

## Behavior

`fetch-archive` declares `FETCH_TIMEOUT`, 30 seconds, in its own module beside
`MAX_FETCH_BYTES` and `MAX_FETCH_REDIRECTS`, and sets it on the client it
builds. The bound does not change; it becomes the runtime's own value, so a
`reqwest` upgrade cannot move it. A request whose response does not arrive
within `FETCH_TIMEOUT`, or whose body stalls that long between reads, fails
with the HTTP error a timeout already produced.

The plan's transport section states the bound as it is: the wait for the
response and each read of its body, not the whole download.

## Edge Cases

- **A slow, steady download** does not time out while each read returns
  within `FETCH_TIMEOUT`. A deadline on the whole download would change
  behavior rather than name it, so this scenario does not add one;
  [fetch-archive-bounds-the-whole-download](fetch-archive-bounds-the-whole-download.md)
  does.
- **A test of the bound** cannot wait 30 seconds. The client is built through
  a seam that takes the timeout, and the test drives it with a short one
  against a proxy that never answers, so the test fails if the client
  ignores the value it is given.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
