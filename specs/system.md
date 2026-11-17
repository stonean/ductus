# System

The architecture of `ductus` as it stands. Each feature's design lives in its own spec; this file holds what a rule asks the project to document in one place.

## Network

The `ductus` runtime is a local stdio process its operator launches, with no listener ([022](022-deterministic-runtime/plan.md)). Its one outbound HTTP client is `fetch-archive`, which fetches over `https` at TLS 1.2 or later (022's plan, §`fetch-archive` transport).

**Loopback plaintext** (`BE-DATA-001`). `fetch-archive` fetches over plain `http` only from a host named in `DUCTUS_FETCH_ALLOW_INSECURE_HOSTS` that is a loopback literal, an address in `127.0.0.0/8` or `::1`: the parity tests' local archive server, or an operator's local test server. That connection stays on the host, inside its trust boundary. Every other host is fetched over `https`, a named one included (022 task 50).
