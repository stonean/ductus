# API Contract Rules — Backend

Contract-quality rules — the shape, stability, and documentation of HTTP and RPC APIs — for any project that exposes a programmatic interface (REST, JSON-RPC, gRPC, GraphQL) to clients outside its own process; HTTP-surface security (security headers, CORS, rate limits, method allowlisting, content-type validation, webhook signing, network exposure) is `security-backend.md` §BE-API. A rule phrased in HTTP methods, status codes, or headers applies to them as HTTP defines them, and reaches an RPC protocol carried over HTTP — gRPC (every call an HTTP/2 `POST`), GraphQL, JSON-RPC — only as far as its own text says (`BE-APIVER-001`, `BE-APIVER-002`, `BE-STATUS-001`, `BE-IDEMP-001`, `BE-COND-001`). These rules verify **design-time commitments** — what a spec or plan must state — rather than code patterns; `/{project}:analyze` enforces them against feature artifacts. Rule IDs follow the format `BE-{CATEGORY}-{NNN}`. Categories: `SCHEMA` (schema publication and source-of-truth), `APIVER` (versioning and deprecation), `ERRENV` (error response envelope), `STATUS` (HTTP status code discipline), `PAGE` (pagination), `IDEMP` (idempotency), `COMPAT` (backwards compatibility), `PATCH` (partial updates), `COND` (conditional requests), `LRO` (long-running operations), `REPR` (representation conventions). Severity, ID permanence, and how review selects or excludes this file follow constitution §rules.

## BE-SCHEMA — Schema Publication

### BE-SCHEMA-001

> Every public HTTP or RPC API MUST publish a machine-readable schema describing every endpoint, request shape, response shape, and error: OpenAPI 3.x for REST, `.proto` files for gRPC, GraphQL SDL for GraphQL, an OpenRPC document (JSON Schema–based) for JSON-RPC. The schema MUST be served from a documented URL on the same origin as the API AND committed to the repository.

**Verification:** Any spec or plan that introduces a public HTTP/RPC endpoint MUST name the schema artifact path AND the URL it is served from. `/{project}:analyze` flags endpoint specs that omit the schema commitment, and flags the absence of a schema file at the named path. **Not flagged:** a GraphQL API that serves its SDL from a documented URL with introspection disabled in production; introspection enabled in production is `BE-INPUT-014`'s.

**Source:** OpenAPI Specification; gRPC documentation, *Introduction to gRPC*; GraphQL Specification (September 2025) §3 (Type System); OpenRPC Specification; OpenAPI Initiative best practices (*Make the OpenAPI Descriptions Available to the Users*, *Add OpenAPI Descriptions to Source Control*)

### BE-SCHEMA-002

> The published schema and the implementation MUST derive from a single source of truth. Either the schema is generated from the code or annotations that implement the endpoints, or the implementation is bound to a design-first schema: generated from it, compiled against it (`.proto`, GraphQL SDL), or validated against it on every request and response or by contract tests. A hand-written schema that the implementation neither derives from nor is checked against MUST NOT be used.

**Verification:** Any spec or plan that introduces or modifies an API endpoint MUST name the schema's single source — a code-derived generation step in the build, or the design-first schema the implementation is bound to. `/{project}:analyze` flags plans that propose a hand-maintained schema bound to the implementation by none of the Statement's means, and affected-files snippets where a generated schema is edited by hand, independently of the code it is generated from.

**Source:** OpenAPI Initiative best practices (*Use a Design-First Approach*, *Keep a Single Source of Truth*)

### BE-SCHEMA-003

> The committed schema MUST match the schema the implementation produces. A change that alters the implementation's contract MUST carry the regenerated schema in the same change, and an implementation whose committed schema diverges from what it would regenerate MUST NOT ship. In a design-first build (`BE-SCHEMA-002`), where the committed schema is the source and nothing regenerates it, an implementation that diverges from the committed schema MUST NOT ship.

**Verification:** Any spec or plan that introduces or modifies an API endpoint, in a project that publishes an API schema under `BE-SCHEMA-001`, MUST commit to updating the committed schema in the same change (or, design-first, to keeping the implementation bound to it) and MUST name how divergence is detected before the change ships. `/{project}:analyze` flags API specs that change the contract without committing to an updated schema, and plans whose affected files change the implementation's contract but not the schema. **Not flagged:** any detection the project chooses, such as a CI job, a pre-commit hook, or a release script; a check not automated to run on every change is `BE-SCHEMA-004`'s.

**Source:** OpenAPI Initiative best practices (*Add OpenAPI Descriptions to Source Control*, *Keep a Single Source of Truth*)

### BE-SCHEMA-004

> The schema-match check of `BE-SCHEMA-003` SHOULD be automated to run on every change proposed for or pushed to the integration branch and to fail that change on divergence — a schema regeneration followed by a diff that exits non-zero on any difference, or, in a design-first build, the step that regenerates server code from the schema, compiles against it, or runs the contract tests.

**Verification:** Any spec or plan covering build or CI configuration, in a project that publishes an API schema under `BE-SCHEMA-001`, SHOULD include an automated schema-drift check. `/{project}:analyze` emits a warning when such a spec omits it. Where the check runs, such as CI, a pre-commit hook, or a merge queue, is the project's choice.

**Source:** OpenAPI Initiative best practices (*Add OpenAPI Descriptions to Source Control*, *Keep a Single Source of Truth*)

## BE-APIVER — Versioning and Deprecation

### BE-APIVER-001

> Every public API MUST declare its versioning strategy explicitly: URI path versioning (`/v1/`, `/v2/`), query-parameter versioning (`?api-version=2024-10-01`), media-type versioning (`Accept: application/vnd.app.v2+json`), header versioning (`API-Version: 2`), or — for gRPC — a major version at the end of the protobuf package (`acme.billing.v1`). The strategy MUST be documented in `specs/system.md` and applied consistently across all endpoints of the API.

**Verification:** Any spec or plan that introduces a public API MUST name the versioning strategy and reference its declaration in `specs/system.md`. `/{project}:analyze` flags public-API specs that omit the strategy commitment, and inconsistencies where some endpoints version one way and others a different way.

**Source:** Microsoft REST API Guidelines (Azure, *API Versioning*), Google AIP-185 (API Versioning)

### BE-APIVER-002

> Deprecated endpoints MUST emit the `Deprecation` (RFC 9745) and `Sunset` (RFC 8594) HTTP response headers. `Deprecation` MUST carry the date the endpoint is or was deprecated, as a Structured Field Date (`Deprecation: @1735689600`). `Sunset` MUST carry, as an HTTP-date, a future date past which the endpoint will be removed, and MUST NOT be earlier than the `Deprecation` date. The corresponding schema definition MUST mark the operation deprecated (OpenAPI `deprecated: true` on the operation, GraphQL `@deprecated` on the field, protobuf `option deprecated = true` on the method, OpenRPC `deprecated: true` on the method). A deprecated GraphQL root field served from an endpoint that is not itself deprecated MAY carry the schema mark alone.

**Verification:** Any spec or plan that removes or deprecates an endpoint MUST commit to the `Deprecation` and `Sunset` headers, with concrete dates in the formats the Statement names, and the schema mark; one that deprecates a GraphQL root field on an endpoint that stays may commit to the mark alone. `/{project}:analyze` flags deprecation specs that omit a required commitment, and a `Sunset` date earlier than the `Deprecation` date.

**Source:** RFC 8594 §3; RFC 9745 §2.1, §2.2, and §4; RFC 9651 §3.3.7; RFC 9110 §5.6.7; OpenAPI Specification (Operation Object `deprecated`); GraphQL Specification (September 2025) §3.13.3; Protocol Buffers `descriptor.proto` (`MethodOptions.deprecated`); OpenRPC Specification (Method Object `deprecated`)

## BE-ERRENV — Error Response Envelope

### BE-ERRENV-001

> Error responses from HTTP APIs MUST use a single documented envelope shape across all endpoints.

**Verification:** Any spec or plan that introduces an HTTP API endpoint MUST commit to the project's error envelope shape and reference its definition in `specs/system.md`. `/{project}:analyze` flags endpoint specs that describe ad-hoc error formats (per-endpoint shapes, mixed error types, naked strings) without naming the project envelope. This rule and `BE-ERRENV-002` govern the envelope's contract shape and code stability: internal detail in a production error response is `BE-ERR-001`'s, and a missing request correlation ID `BE-ERR-002`'s.

**Source:** RFC 9457 (Problem Details for HTTP APIs, obsoletes RFC 7807), Google AIP-193 (Errors)

### BE-ERRENV-002

> The error envelope MUST include a machine-readable, stable error code distinct from the HTTP status code (e.g., `code: "USER_NOT_FOUND"`, or an RFC 9457 `type` URI). Error codes MUST be documented in the schema, MUST be stable across versions, and MUST NOT be derived from or parsed out of the human-readable message.

**Verification:** Any spec or plan that introduces an HTTP API endpoint MUST commit to documented error codes for each named failure mode, with the code values declared in the schema. `/{project}:analyze` flags endpoint specs that describe errors only by HTTP status or only by message text without a stable code.

**Source:** Stripe API error documentation, RFC 9457 §3.1.1, Google AIP-193

### BE-ERRENV-003

> The error envelope SHOULD be RFC 9457 `application/problem+json`, with at least the `type`, `title`, `status`, and `detail` fields. A project-specific envelope is acceptable when it is documented in `specs/system.md` and used uniformly.

**Verification:** Any spec or plan that defines the project's error envelope SHOULD commit to RFC 9457 `application/problem+json` or name the project-specific envelope documented in `specs/system.md` that replaces it. `/{project}:analyze` emits a warning when an envelope definition is neither.

**Source:** RFC 9457 (Problem Details for HTTP APIs, obsoletes RFC 7807)

## BE-STATUS — HTTP Status Code Discipline

### BE-STATUS-001

> HTTP APIs MUST use status codes with their registered semantics — RFC 9110, plus RFC 6585 for `429`. Required mappings: `200` successful retrieval, `201` resource creation (with `Location` header naming the new resource), `202` accepted-but-not-yet-processed, `204` successful no-content, `400` malformed request, `401` missing/invalid authentication (with the `WWW-Authenticate` challenge RFC 9110 requires), `403` authenticated-but-not-authorized (or `404` where the resource's existence must not be revealed, per `BE-AUTHZ-006`), `404` resource-not-found, `409` conflict, `410` permanently-removed, `422` well-formed-but-semantically-invalid input, `429` rate-limited, `500` unexpected server error, `503` unavailable-with-retry. Endpoints MUST NOT collapse multiple outcomes into `200` with an `error` field in the body. RPC protocols carried over HTTP MAY instead signal errors as their own specifications define — GraphQL per the GraphQL over HTTP working draft, JSON-RPC through its error object, gRPC through `grpc-status`.

**Verification:** Any spec or plan that introduces an HTTP endpoint MUST name the success status code AND each failure status code mapped to its failure mode — or, for an RPC protocol carried over HTTP, the error its protocol signals for each failure mode. `/{project}:analyze` flags endpoint specs that describe errors via `200 OK` with an `error` field, that conflate `401` and `403`, that describe a `401` without a `WWW-Authenticate` challenge, or that omit the status-code mapping.

**Source:** RFC 9110 §15 (§15.5.2 for `401`, §15.5.4 for hiding existence behind `404`), RFC 6585 §4, Microsoft REST API Guidelines (Azure, *HTTP Return Codes*), GraphQL over HTTP (working draft), JSON-RPC 2.0 Specification (5.1 Error object), gRPC *PROTOCOL-HTTP2* (`:method POST`, `:status 200`, `grpc-status` in trailers)

## BE-PAGE — Pagination

### BE-PAGE-001

> List endpoints that can return more than a small fixed maximum number of items MUST paginate, with a default and a maximum page size. The pagination strategy MUST be documented per endpoint.

**Verification:** Any spec or plan that introduces a list endpoint MUST commit to a pagination strategy, a default page size, and a maximum page size, with both sizes as named constants per `CFG-CONST-003`. `/{project}:analyze` flags list-endpoint specs that omit pagination or leave the strategy unnamed.

**Source:** Stripe API pagination, GraphQL Cursor Connections Specification

### BE-PAGE-002

> Paginated responses MUST include an envelope distinguishing the page from the items, with an explicit continuation field. The presence of more pages MUST be expressed by a dedicated field — not inferred from a full-page item count.

**Verification:** Any spec or plan that introduces a paginated endpoint MUST name the response envelope including the page-continuation field. `/{project}:analyze` flags paginated-endpoint specs that infer continuation from item count or that omit a continuation field. **Not flagged:** a continuation field such as `has_more`, or a `next_cursor` that is null on the last page.

**Source:** Stripe API pagination, GraphQL Cursor Connections Specification

### BE-PAGE-003

> Pagination SHOULD default to cursor-based pagination (opaque continuation tokens) for any collection that supports concurrent inserts or deletes. Offset-based pagination MAY be used only for static or append-only-at-tail collections.

**Verification:** Any spec or plan that introduces a list endpoint over a collection that accepts concurrent inserts or deletes SHOULD commit to cursor-based pagination; `/{project}:analyze` emits a warning when such a spec proposes offset pagination. **Not flagged:** offset pagination over a static or append-only-at-tail collection, which the Statement permits.

**Source:** Stripe API pagination, GraphQL Cursor Connections Specification

### BE-PAGE-004

> Paginated list endpoints MUST return items in a deterministic total order: a documented sort whose final key is unique (e.g., `created_at, id`), applied identically on every page.

**Verification:** Any spec or plan that introduces a paginated endpoint MUST name its sort order, ending in a unique key. `/{project}:analyze` flags paginated-endpoint specs that leave the order unstated or that sort only on a non-unique column (a timestamp, a name, a status).

**Source:** PostgreSQL documentation, *LIMIT and OFFSET*; GraphQL Cursor Connections Specification; Microsoft REST API Guidelines (Azure, *Considerations for sorting with pagination*)

## BE-IDEMP — Idempotency

### BE-IDEMP-001

> Endpoints that create resources or trigger side-effecting state changes via `POST`, or via a `PATCH` whose patch is not idempotent (e.g., a JSON Patch `add` to an array), MUST accept an `Idempotency-Key` request header. The server MUST persist key→response mappings for at least 24 hours and return the cached response on retry with the same key (matching auth principal, matching request body hash). A request that reuses a key with a different body MUST be rejected with `422 Unprocessable Content`, and a retry that arrives while the original request is still in flight MUST be rejected with `409 Conflict`. A side-effecting call of an RPC protocol carried over HTTP — a gRPC method, a GraphQL mutation, a JSON-RPC method — counts as a `POST` under this rule; it MAY carry the key in a request field instead of the header (e.g., `request_id` per AIP-155) and MAY signal the two rejections through its protocol's error channel, as `BE-STATUS-001` permits.

**Verification:** Any spec or plan that introduces a side-effecting `POST` endpoint — including a side-effecting RPC method carried over HTTP — or a `PATCH` endpoint whose patch is not idempotent, MUST commit to the Statement's key carrier, persistence window, and two rejection responses. `/{project}:analyze` flags such specs that omit idempotency support or either response, and persistence windows shorter than the documented client retry policy.

**Source:** Stripe API idempotency documentation (API v1); IETF draft-ietf-httpapi-idempotency-key-header-07 §2.4, §2.6–§2.7, §4, and §5 (an Internet-Draft that expired in April 2026 without RFC publication); Google AIP-155 (Request identification)

### BE-IDEMP-002

> `PUT` and `DELETE` operations MUST be idempotent in the HTTP sense: repeated identical requests MUST have the same intended effect on the resource as a single request. The response MAY differ between attempts (`201` then `200` for a `PUT` that creates, `204` then `404` for a `DELETE` retried once the resource is gone). Business side effects of the change (counter increments, notification sends, charges, outbound events) MUST NOT be repeated on `PUT`/`DELETE` retries; per-request logging and revision history MAY still record every attempt.

**Verification:** Any spec or plan that introduces a `PUT` or `DELETE` endpoint MUST commit to operation idempotency, including deduplication of business side effects on retries. `/{project}:analyze` flags `PUT`/`DELETE` specs that describe per-call counter increments, notification sends, charges, or outbound events without dedup. **Not flagged:** per-request logging, including the audit logging `security-backend.md` §BE-LOG requires, and revision history.

**Source:** RFC 9110 §9.2.2

## BE-COMPAT — Backwards Compatibility

### BE-COMPAT-001

> Within a major version, schema changes MUST be additive only. Removing or renaming a field, changing a field's type (including narrowing, e.g. `string` → `enum`), changing a default value, adding a required request field or making an optional request field required, ceasing to populate a response field the server previously populated, or adding a value to a response enum the schema does not document as open to new values MUST NOT happen without a major version bump. Adding new optional request fields, new response fields, new values to enums used only in requests, new values to response enums the schema documents as open to new values, and new endpoints is permitted.

**Verification:** Any spec or plan that modifies an existing API endpoint MUST classify the change as additive (compatible) or breaking (requires major version). `/{project}:analyze` flags endpoint-modifying specs that omit the classification, and plans that describe a change the Statement lists as breaking — subtractive, narrowing, or otherwise — within a major version without a major-version commitment.

**Source:** Semantic Versioning 2.0.0, Google AIP-180 (Backwards compatibility)

## BE-PATCH — Partial Updates

### BE-PATCH-001

> `PATCH` endpoints MUST declare their patch document format by media type — JSON Merge Patch (`application/merge-patch+json`, RFC 7396), JSON Patch (`application/json-patch+json`, RFC 6902), or a project format documented in `specs/system.md` that defines how an absent field and an explicit `null` each apply. The server MUST apply a patch atomically: when any change in the document cannot be applied, none is applied, and no partially modified representation is ever observable.

**Verification:** Any spec or plan that introduces a `PATCH` endpoint MUST name its patch media type (or the documented project format) and commit to all-or-nothing application. `/{project}:analyze` flags `PATCH` specs that accept a plain `application/json` body with no stated absent-versus-`null` semantics, and plans that apply patch operations one by one without a transaction or equivalent all-or-nothing guard.

**Source:** RFC 5789 §2, RFC 7396, RFC 6902, Microsoft REST API Guidelines (Azure, *HTTP Return Codes* and *JSON*)

## BE-COND — Conditional Requests

### BE-COND-001

> For a resource more than one client can modify, a write that replaces it, or that applies a change whose effect depends on the representation the client read, MUST let the client make the write conditional on the version it read — whether the write is a `PUT`, such a `PATCH`, or an RPC update method or GraphQL mutation. The mechanism is either HTTP conditional requests — a validator (`ETag`) on responses, `If-Match` honored on the write, and `412 Precondition Failed` on a mismatch — or a version field the schema documents (e.g., the `etag` field of AIP-154), compared by the server before applying the write, with a mismatch rejected with `409 Conflict` or, in an RPC protocol, its equivalent error (gRPC `ABORTED`).

**Verification:** Any spec or plan that introduces, on a resource multiple clients can modify, a `PUT` endpoint, a `PATCH` endpoint whose patch depends on the prior representation, or an RPC update method or GraphQL mutation that replaces the resource or applies such a change MUST name its precondition mechanism and its mismatch response. `/{project}:analyze` flags such specs that commit to no precondition mechanism, including one whose only guard is a row lock or transaction held inside the write. A lost update within one request — the server's own read-modify-write with no guard — is `concurrency-backend.md` `BE-TXN-002`'s.

**Source:** RFC 9110 §8.8.3 (`ETag`), §13.1.1 (`If-Match`), §15.5.10 (`409` for a version conflict), §15.5.13 (`412`); RFC 5789 §2; Google AIP-154 (Resource freshness validation); Microsoft REST API Guidelines (Azure, *Conditional Requests*)

## BE-LRO — Long-Running Operations

### BE-LRO-001

> A `202 Accepted` response SHOULD identify a status monitor — a `Location` or `Operation-Location` header, or a documented field in the response body, naming a resource that reports the operation's state (pending, running, succeeded, failed). That resource SHOULD expose the result, or a link to it, on success and the failure in the project's error envelope (`BE-ERRENV-001`) on failure, and SHOULD carry `Retry-After` as the polling interval while the operation is incomplete. A `202` whose completion is delivered through a documented push channel (a webhook, server-sent events) MAY identify that channel instead of a status monitor.

**Verification:** Any spec or plan that introduces an endpoint returning `202 Accepted` SHOULD name the status-monitor URL, the operation states it reports, how the result and a failure are retrieved, and its `Retry-After` polling interval, or the documented push channel the Statement's MAY allows. `/{project}:analyze` emits a warning when a `202` endpoint spec names neither a status monitor nor a push channel — except where the client needs the outcome and has no way to observe completion, which `performance-backend.md`'s `BE-ASYNC-001` owns — or names a status monitor that reports no operation states, gives no way to retrieve the result, or none to retrieve the failure, omits `Retry-After`, or reports failure outside the project error envelope. This rule governs the status monitor's shape: slow work left on the request path, or offloaded with no way for a client that needs the outcome to observe completion, is `BE-ASYNC-001`'s.

**Source:** RFC 9110 §15.3.3 and §10.2.3 (`Retry-After`), Microsoft REST API Guidelines (Azure, *Long-Running Operations & Jobs*), Google AIP-151 (Long-running operations)

## BE-REPR — Representation Conventions

### BE-REPR-001

> Instants (timestamps) in request and response payloads MUST be unambiguous about time zone and represented one way across the API: RFC 3339 date-time strings carrying an explicit UTC offset (`2026-10-19T14:30:00Z`, `2026-10-19T09:30:00-05:00`), or integer Unix time with its unit (seconds or milliseconds) documented in the schema, or a schema type defined as a count from the Unix epoch (protobuf `google.protobuf.Timestamp`). Date-times without an offset MUST NOT represent instants. Calendar dates and local times that denote no instant (a birthday, an opening hour) MAY use RFC 3339 `full-date` and `partial-time`.

**Verification:** Any spec or plan that introduces a timestamp field in an API request or response MUST name its representation, and `specs/system.md` MUST declare the API-wide one. `/{project}:analyze` flags timestamp fields described without a representation, with an offset-less date-time format, or with a representation that differs from the one `specs/system.md` declares.

**Source:** RFC 3339 §5.6, Microsoft REST API Guidelines (Azure, *JSON*), Google AIP-142 (Time and duration), POSIX.1-2024 Base Definitions 3.125 (*Epoch*), Protocol Buffers `timestamp.proto`

### BE-REPR-002

> Field names, query-parameter names, and enum values SHOULD each follow one casing convention (e.g., `camelCase` field names, `UPPER_SNAKE_CASE` enum values) declared in `specs/system.md` and applied across every endpoint of the API.

**Verification:** Any spec or plan that introduces API fields, query parameters, or enum values SHOULD use the casing convention declared in `specs/system.md`. `/{project}:analyze` emits a warning when a spec's names mix conventions or depart from the declared one, or when a project with a public API declares none.

**Source:** Microsoft REST API Guidelines (Azure, *JSON*), Google AIP-140 (Field names)

### BE-REPR-003

> A JSON payload SHOULD NOT carry a number that a binary64 (double-precision) parser cannot hold exactly: an integer outside the range −(2^53 − 1) to 2^53 − 1, such as a 64-bit identifier, or a value with more precision than binary64 provides. Such a value SHOULD be encoded as a JSON string, documented in the schema as a string-encoded number.

**Verification:** Any spec or plan that introduces a JSON API field holding a number a binary64 parser cannot hold exactly SHOULD encode and document it as the Statement requires. `/{project}:analyze` emits a warning when a spec declares such a field — an `int64` or `uint64` identifier, a high-precision decimal — as a JSON number.

**Source:** RFC 7493 §2.2, RFC 8259 §6
