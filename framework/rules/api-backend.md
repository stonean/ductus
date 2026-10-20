# API Contract Rules — Backend

Enforceable rules for the shape, stability, and documentation of HTTP and RPC APIs. These rules apply to any project that exposes a programmatic interface (REST, JSON-RPC, gRPC, GraphQL) to clients outside its own process.

These rules cover **contract quality**, not security. For HTTP-surface security concerns (security headers, CORS, rate limits, method allowlisting, content-type validation, webhook signing, network exposure) see `security-backend.md` §BE-API.

A rule phrased in HTTP methods, status codes, or headers applies to them as HTTP defines them, and reaches an RPC protocol carried over HTTP — gRPC (every call an HTTP/2 `POST`), GraphQL, JSON-RPC — only as far as its own text says (`BE-APIVER-001`, `BE-APIVER-002`, `BE-STATUS-001`, `BE-IDEMP-001`, `BE-COND-001`).

Rules use RFC 2119 language: **MUST** / **MUST NOT** are enforced as errors; **SHOULD** / **SHOULD NOT** are flagged as warnings.

Rule IDs follow the format `BE-{CATEGORY}-{NNN}` and are permanent — once assigned, an ID is never renumbered, even if the rule is moved within the file or deprecated. Categories: `SCHEMA` (schema publication and source-of-truth), `APIVER` (versioning and deprecation), `ERRENV` (error response envelope), `STATUS` (HTTP status code discipline), `PAGE` (pagination), `IDEMP` (idempotency), `COMPAT` (backwards compatibility), `PATCH` (partial updates), `COND` (conditional requests), `LRO` (long-running operations), `REPR` (representation conventions). See `specs/008-security-rules/data-model.md` for the full schema.

These rules verify **design-time commitments** — what a spec or plan must state — rather than code patterns; `/{project}:analyze` enforces them against feature artifacts.

Projects without a programmatic API need not opt out: `/{project}:analyze` loads every rule file present, regardless of stack or `[rules] surfaces`, and these rules fire only on a spec or plan that meets their Verification trigger, every one of which presupposes a programmatic interface. In `.ductus/config.toml`, a `[rules] surfaces` list without `backend` stops `ductus` from installing or updating this file (a copy already on disk is left in place) and stops `/{project}:review` from selecting it; a `[[review.disabled-rule-files]]` entry with its mandatory `reason` excludes it from `/{project}:review` alone; and listing it under `[pinned] files` only stops `ductus` updates from overwriting a customized copy, which still loads for review and analyze.

## BE-SCHEMA — Schema Publication

### BE-SCHEMA-001

> Every public HTTP or RPC API MUST publish a machine-readable schema describing every endpoint, request shape, response shape, and error: OpenAPI 3.x for REST, `.proto` files for gRPC, GraphQL SDL for GraphQL, an OpenRPC document (JSON Schema–based) for JSON-RPC. The schema MUST be served from a documented URL on the same origin as the API (e.g., `/openapi.json`, `/schema.graphql`) AND committed to the repository.

**Rationale:** A machine-readable schema is the single source of truth that client SDKs, mock servers, contract tests, and human documentation all derive from. Without it, every consumer reverse-engineers the API independently and integration breaks accumulate silently. Serving it from the API origin makes discovery automatic. Publishing the schema is distinct from GraphQL introspection: serving the SDL from a documented URL satisfies this rule while production introspection stays disabled per `BE-INPUT-014`.

**Verification:** Any spec or plan that introduces a public HTTP/RPC endpoint MUST name the schema artifact path AND the URL it is served from. `/{project}:analyze` flags endpoint specs that omit the schema commitment, and flags the absence of a schema file at the named path.

**Source:** OpenAPI Specification; gRPC documentation, *Introduction to gRPC* (Protocol Buffers as the IDL); GraphQL Specification (September 2025) §3 (Type System); OpenRPC Specification (Schema Objects follow JSON Schema draft 7); OpenAPI Initiative best practices (*Make the OpenAPI Descriptions Available to the Users*, *Add OpenAPI Descriptions to Source Control*)

### BE-SCHEMA-002

> The published schema and the implementation MUST derive from a single source of truth — never maintained as parallel documents. Either the schema is generated from the code or annotations that implement the endpoints (typed handler signatures and decorators in FastAPI, NestJS, ASP.NET, Spring, or build-time extraction from a typed routing layer), or the implementation is bound to a design-first schema: generated from it, compiled against it (`.proto`, GraphQL SDL), or validated against it on every request and response or by contract tests. A hand-written schema that the implementation neither derives from nor is checked against MUST NOT be used.

**Rationale:** Parallel schemas drift the moment a parameter is renamed, an optional field is made required, or a status code is added — and the drift is silent because the schema never executes. A schema bound to the implementation in either direction fails loudly when implementation and contract diverge, and is as trustworthy as the code itself. The OpenAPI Initiative recommends design-first; this rule accepts it, and code-first, on the same condition.

**Verification:** Any spec or plan that introduces or modifies an API endpoint MUST name the schema's single source — a code-derived generation step in the build, or the design-first schema the implementation is generated from, compiled against, or validated against. `/{project}:analyze` flags plans that propose a hand-maintained schema bound to the implementation by none of these, and flags affected-files snippets where a generated schema is edited by hand, independently of the code it is generated from.

**Source:** OpenAPI Initiative best practices (*Use a Design-First Approach*, *Keep a Single Source of Truth*)

### BE-SCHEMA-003

> The committed schema MUST match the schema the implementation produces. A change that alters the implementation's contract MUST carry the regenerated schema in the same change, and an implementation whose committed schema diverges from what it would regenerate MUST NOT ship. In a design-first build (`BE-SCHEMA-002`), where the committed schema is the source and nothing regenerates it, an implementation that diverges from the committed schema MUST NOT ship.

**Rationale:** Generated schemas drift the moment a developer skips the regeneration step, and the drift is silent until a client breaks against the published contract. Holding what ships to a matching schema makes drift a defect fixed before release instead of a production discovery. How the match is checked — a CI job, a pre-commit hook, a release script — is the project's choice, since the automation a project runs is its own; `BE-SCHEMA-004` recommends automating the check.

**Verification:** Any spec or plan that introduces or modifies an API endpoint, in a project that publishes an API schema under `BE-SCHEMA-001`, MUST commit to updating the committed schema in the same change (or, design-first, to keeping the implementation bound to it) and MUST name how divergence is detected before the change ships. `/{project}:analyze` flags API specs that change the contract without committing to an updated schema, and plans whose affected files change the implementation's contract but not the schema.

**Source:** OpenAPI Initiative best practices (*Add OpenAPI Descriptions to Source Control*, *Keep a Single Source of Truth*)

### BE-SCHEMA-004

> The schema-match check of `BE-SCHEMA-003` SHOULD be automated to run on every change proposed for or pushed to the integration branch and to fail that change on divergence — a schema regeneration followed by a diff that exits non-zero on any difference, or, in a design-first build, the step that regenerates server code from the schema, compiles against it, or runs the contract tests.

**Rationale:** A manual regeneration step is skipped exactly when a developer is in a hurry; an automated check surfaces drift on the change that introduced it, when it is cheapest to fix. Where the automation runs — CI, a pre-commit hook, a merge queue — is adopter-owned and outside what a spec or plan can bind, so this rule is advisory.

**Verification:** Any spec or plan covering build or CI configuration, in a project that publishes an API schema under `BE-SCHEMA-001`, SHOULD include an automated schema-drift check. `/{project}:analyze` emits a warning when such a spec omits it.

**Source:** OpenAPI Initiative best practices (*Add OpenAPI Descriptions to Source Control*, *Keep a Single Source of Truth*)

## BE-APIVER — Versioning and Deprecation

### BE-APIVER-001

> Every public API MUST declare its versioning strategy explicitly: URI path versioning (`/v1/`, `/v2/`), query-parameter versioning (`?api-version=2024-10-01`), media-type versioning (`Accept: application/vnd.app.v2+json`), header versioning (`API-Version: 2`), or — for gRPC — a major version at the end of the protobuf package (`acme.billing.v1`). The strategy MUST be documented in `specs/system.md` and applied consistently across all endpoints of the API.

**Rationale:** Without a declared strategy, breaking changes have no migration path — every consumer breaks simultaneously when an endpoint changes. A declared strategy lets old and new versions coexist while consumers migrate at their own pace.

**Verification:** Any spec or plan that introduces a public API MUST name the versioning strategy and reference its declaration in `specs/system.md`. `/{project}:analyze` flags public-API specs that omit the strategy commitment, and flags inconsistencies where some endpoints version one way and others a different way.

**Source:** Microsoft REST API Guidelines (Azure, *API Versioning*), Google AIP-185 (API Versioning; the Cloud API Design Guide's versioning page now redirects to it)

### BE-APIVER-002

> Deprecated endpoints MUST emit the `Deprecation` (RFC 9745) and `Sunset` (RFC 8594) HTTP response headers. `Deprecation` MUST carry the date the endpoint is or was deprecated, as a Structured Field Date (`Deprecation: @1735689600`). `Sunset` MUST carry, as an HTTP-date, a future date past which the endpoint will be removed, and MUST NOT be earlier than the `Deprecation` date. The corresponding schema definition MUST mark the operation deprecated (OpenAPI `deprecated: true` on the operation, GraphQL `@deprecated` on the field, protobuf `option deprecated = true` on the method, OpenRPC `deprecated: true` on the method). A deprecated GraphQL root field served from an endpoint that is not itself deprecated MAY carry the schema mark alone.

**Rationale:** Consumers cannot plan migrations they do not know about. Emitting the headers on every response from a deprecated endpoint puts the deprecation in client logs, monitoring dashboards, and integration tests. The schema annotation records the deprecation in the contract itself, where generated documentation shows it and a code generator that honors it can mark the generated client method deprecated — for protobuf, `descriptor.proto` says the option can emit a deprecation annotation or be ignored, depending on the target platform. The two headers use different date formats — `Deprecation` a Structured Field Date (`@` plus seconds since 1970-01-01T00:00:00Z), `Sunset` an HTTP-date, which a sender generates as an IMF-fixdate — and a value in the wrong format is unparseable to the tooling that reads it. RFC 9745 scopes `Deprecation`, by default, to the resource the response belongs to, so on a GraphQL endpoint that serves fields still in use, the header would announce the whole endpoint deprecated; `@deprecated` on the field is the signal there.

**Verification:** Any spec or plan that removes or deprecates an endpoint MUST commit to (a) `Deprecation` + `Sunset` headers with concrete dates in their respective formats, `Sunset` no earlier than `Deprecation`, and (b) the schema-level deprecation annotation; one that deprecates a GraphQL root field on an endpoint that stays may commit to (b) alone. `/{project}:analyze` flags deprecation specs that omit a required commitment, and flags a `Sunset` date earlier than the `Deprecation` date.

**Source:** RFC 8594 §3; RFC 9745 §2.1, §2.2, and §4; RFC 9651 §3.3.7; RFC 9110 §5.6.7; OpenAPI Specification (Operation Object `deprecated`); GraphQL Specification (September 2025) §3.13.3; Protocol Buffers `descriptor.proto` (`MethodOptions.deprecated`); OpenRPC Specification (Method Object `deprecated`)

## BE-ERRENV — Error Response Envelope

### BE-ERRENV-001

> Error responses from HTTP APIs MUST use a single documented envelope shape across all endpoints.

**Rationale:** Without a uniform shape, every consumer writes per-endpoint error parsing — and gets it wrong for endpoints added later. A single shape lets clients write one error handler that works everywhere, and lets the API evolve error categories without breaking consumers. The security properties of the same envelope — no internal detail in production, a correlation ID surfaced to the client — are governed by `security-backend.md` §BE-ERR (`BE-ERR-001`, `BE-ERR-002`); this rule and `BE-ERRENV-002` govern its contract shape and code stability.

**Verification:** Any spec or plan that introduces an HTTP API endpoint MUST commit to the project's error envelope shape and reference its definition in `specs/system.md`. `/{project}:analyze` flags endpoint specs that describe ad-hoc error formats (per-endpoint shapes, mixed error types, naked strings) without naming the project envelope.

**Source:** RFC 9457 (Problem Details for HTTP APIs, obsoletes RFC 7807), Google AIP-193 (Errors; the Cloud API Design Guide's errors page now redirects to it)

### BE-ERRENV-002

> The error envelope MUST include a machine-readable, stable error code distinct from the HTTP status code (e.g., `code: "USER_NOT_FOUND"`, `code: "PAYMENT_DECLINED_INSUFFICIENT_FUNDS"`, or an RFC 9457 `type` URI such as `https://api.example.com/problems/user-not-found`). Error codes MUST be documented in the schema, MUST be stable across versions, and MUST NOT be derived from or parsed out of the human-readable message.

**Rationale:** HTTP status codes are coarse — `400` covers thousands of failure modes. Stable error codes let clients write conditional logic ("retry on `RATE_LIMITED`, surface to user on `VALIDATION_FAILED`") that survives message rewording and localization. Deriving codes from messages couples the two and breaks clients when copy is edited.

**Verification:** Any spec or plan that introduces an HTTP API endpoint MUST commit to documented error codes for each named failure mode, with the code values declared in the schema. `/{project}:analyze` flags endpoint specs that describe errors only by HTTP status or only by message text without a stable code.

**Source:** Stripe API error documentation, RFC 9457 §3.1.1, Google AIP-193

### BE-ERRENV-003

> The error envelope SHOULD be RFC 9457 `application/problem+json`, with at least the `type`, `title`, `status`, and `detail` fields. A project-specific envelope is acceptable when it is documented in `specs/system.md` and used uniformly.

**Rationale:** A standard envelope lets clients and shared tooling — HTTP client libraries, API gateways, observability pipelines — parse errors without project-specific code, and it gives every new endpoint a documented default rather than a fresh design decision. A documented project envelope serves the same purpose where the standard one does not fit, provided it is used as uniformly as `BE-ERRENV-001` requires.

**Verification:** Any spec or plan that defines the project's error envelope SHOULD commit to RFC 9457 `application/problem+json` or name the project-specific envelope documented in `specs/system.md` that replaces it. `/{project}:analyze` emits a warning when an envelope definition is neither RFC 9457 nor a documented project envelope.

**Source:** RFC 9457 (Problem Details for HTTP APIs, obsoletes RFC 7807)

## BE-STATUS — HTTP Status Code Discipline

### BE-STATUS-001

> HTTP APIs MUST use status codes with their registered semantics — RFC 9110, plus RFC 6585 for `429`. Required mappings: `200` successful retrieval, `201` resource creation (with `Location` header naming the new resource), `202` accepted-but-not-yet-processed, `204` successful no-content, `400` malformed request, `401` missing/invalid authentication (with the `WWW-Authenticate` challenge RFC 9110 requires), `403` authenticated-but-not-authorized (or `404` where the resource's existence must not be revealed, per `BE-AUTHZ-006`), `404` resource-not-found, `409` conflict, `410` permanently-removed, `422` well-formed-but-semantically-invalid input, `429` rate-limited, `500` unexpected server error, `503` unavailable-with-retry. Endpoints MUST NOT collapse multiple outcomes into `200` with an `error` field in the body. RPC protocols carried over HTTP MAY instead signal errors as their own specifications define — GraphQL per the GraphQL over HTTP working draft, JSON-RPC through its error object, gRPC through `grpc-status`.

**Rationale:** HTTP intermediaries — load balancers, caches, retry middleware, monitoring, on-call alerting — all act on status codes. An endpoint that returns `200` with an `error` field bypasses every layer of the HTTP-aware stack and forces every consumer (and operator) to parse the body to know what happened. Correct status codes let off-the-shelf infrastructure do its job. RPC protocols layered on HTTP define their own error channel — a GraphQL `errors` entry, a JSON-RPC error object, a gRPC status sent in trailers on an HTTP `200` response — which their clients and tooling read instead, so the rule defers to those specifications rather than forcing these mappings onto them.

**Verification:** Any spec or plan that introduces an HTTP endpoint MUST name the success status code AND each failure status code mapped to its failure mode — or, for an RPC protocol carried over HTTP, the error its protocol signals for each failure mode. `/{project}:analyze` flags endpoint specs that describe errors via `200 OK` with an `error` field, that conflate `401` and `403`, that describe a `401` without a `WWW-Authenticate` challenge, or that omit the status-code mapping.

**Source:** RFC 9110 §15 (§15.5.2 for `401`, §15.5.4 for hiding existence behind `404`), RFC 6585 §4, Microsoft REST API Guidelines (Azure, *HTTP Return Codes*), GraphQL over HTTP (working draft), JSON-RPC 2.0 Specification (5.1 Error object), gRPC *PROTOCOL-HTTP2* (`:method POST`, `:status 200`, `grpc-status` in trailers)

## BE-PAGE — Pagination

### BE-PAGE-001

> List endpoints that can return more than a small fixed maximum number of items MUST paginate, with a default and a maximum page size. The pagination strategy MUST be documented per endpoint.

**Rationale:** Unpaginated list endpoints are a DoS vector (large responses, slow queries) and an OOM-on-client risk. A strategy documented per endpoint tells clients how to walk the collection and what the walk guarantees when the collection changes underneath it.

**Verification:** Any spec or plan that introduces a list endpoint MUST commit to a pagination strategy, a default page size, and a maximum page size, with both sizes as named constants per `CFG-CONST-003`. `/{project}:analyze` flags list-endpoint specs that omit pagination or leave the strategy unnamed.

**Source:** Stripe API pagination, GraphQL Cursor Connections Specification

### BE-PAGE-002

> Paginated responses MUST include an envelope distinguishing the page from the items, with an explicit continuation field: e.g., `{"data": [...], "next_cursor": "..." | null, "has_more": true | false}`. The presence of more pages MUST be expressed by a dedicated field — not inferred from a full-page item count.

**Rationale:** Inferring "more pages exist when `len(items) == page_size`" is wrong at the exact boundary where the total happens to equal the page size — the client requests an empty page on the next call. A dedicated `has_more` (or non-null `next_cursor`) makes the boundary unambiguous and saves one round trip.

**Verification:** Any spec or plan that introduces a paginated endpoint MUST name the response envelope including the page-continuation field. `/{project}:analyze` flags paginated-endpoint specs that infer continuation from item count or that omit a continuation field.

**Source:** Stripe API pagination, GraphQL Cursor Connections Specification

### BE-PAGE-003

> Pagination SHOULD default to cursor-based pagination (opaque continuation tokens) for any collection that supports concurrent inserts or deletes. Offset-based pagination MAY be used only for static or append-only-at-tail collections.

**Rationale:** Under offset pagination, a row inserted ahead of the current offset between page reads shifts later rows down, so the next page repeats a row already returned, and a row deleted ahead of it shifts them up, so the next page skips one; a cursor anchored to the last item returned is unaffected by either. Defaulting to cursors forces the safe choice without per-endpoint debate.

**Verification:** Any spec or plan that introduces a list endpoint over a collection that accepts concurrent inserts or deletes SHOULD commit to cursor-based pagination. `/{project}:analyze` emits a warning when a list-endpoint spec proposes offset pagination on a collection that accepts concurrent inserts or deletes.

**Source:** Stripe API pagination, GraphQL Cursor Connections Specification

### BE-PAGE-004

> Paginated list endpoints MUST return items in a deterministic total order: a documented sort whose final key is unique (e.g., `created_at, id`), applied identically on every page.

**Rationale:** Without a total order, the database may return rows that tie on the sort key in any order, and that order can change from one query to the next — so an item appears on two pages or on none, under offset and cursor pagination alike. Ending the sort with a unique key makes every position well-defined and every page boundary reproducible; a cursor that encodes the last item's sort-key values is only stable when those values identify exactly one item.

**Verification:** Any spec or plan that introduces a paginated endpoint MUST name its sort order, ending in a unique key. `/{project}:analyze` flags paginated-endpoint specs that leave the order unstated or that sort only on a non-unique column (a timestamp, a name, a status).

**Source:** PostgreSQL documentation, *LIMIT and OFFSET* (an ORDER BY that does not constrain rows into a unique order yields an unpredictable subset); GraphQL Cursor Connections Specification (edge ordering consistent from page to page); Microsoft REST API Guidelines (Azure, *Considerations for sorting with pagination*)

## BE-IDEMP — Idempotency

### BE-IDEMP-001

> Endpoints that create resources or trigger side-effecting state changes via `POST`, or via a `PATCH` whose patch is not idempotent (e.g., a JSON Patch `add` to an array), MUST accept an `Idempotency-Key` request header. The server MUST persist key→response mappings for at least 24 hours and return the cached response on retry with the same key (matching auth principal, matching request body hash). A request that reuses a key with a different body MUST be rejected with `422 Unprocessable Content`, and a retry that arrives while the original request is still in flight MUST be rejected with `409 Conflict`. A side-effecting call of an RPC protocol carried over HTTP — a gRPC method, a GraphQL mutation, a JSON-RPC method — counts as a `POST` under this rule; it MAY carry the key in a request field instead of the header (e.g., `request_id` per AIP-155) and MAY signal the two rejections through its protocol's error channel, as `BE-STATUS-001` permits.

**Rationale:** Networks fail mid-request. Without idempotency, a client that did not receive a response cannot safely retry — every retry risks a duplicate side effect (double-charge, double-send, duplicate order). The `Idempotency-Key` header makes safe retry possible: Stripe's v1 API accepts it on every `POST`, and the IETF draft that would standardize it records implementations by Stripe, Adyen, Dwolla, and others. A key reused with a different body is a client error rather than a retry, so it is rejected instead of served the cached response; a duplicate that races the original is rejected so the side effect cannot run twice. An RPC call over HTTP carries the same hazard on the same method — every gRPC call is an HTTP/2 `POST`, and the GraphQL over HTTP working draft forbids executing a mutation by `GET` — but its protocol signals errors in its own channel (a gRPC status travels in trailers on an HTTP `200`), and RPC conventions such as AIP-155 carry the request identifier in the request message, so the rule takes the protection and lets the protocol choose the carrier.

**Verification:** Any spec or plan that introduces a side-effecting `POST` endpoint — including a side-effecting RPC method carried over HTTP — or a `PATCH` endpoint whose patch is not idempotent, MUST commit to `Idempotency-Key` support (or, for an RPC method, the header or request field that carries the key), the persistence window, the reused-key-different-body response (`422`, or the RPC protocol's error), and the in-flight-duplicate response (`409`, or the RPC protocol's error). `/{project}:analyze` flags such specs that omit idempotency support or either response, and flags persistence windows that are shorter than the documented client retry policy.

**Source:** Stripe API idempotency documentation (API v1); IETF draft-ietf-httpapi-idempotency-key-header-07 §2.4, §2.6–§2.7, §4, and §5 (an Internet-Draft that expired in April 2026 without RFC publication); Google AIP-155 (Request identification); gRPC *PROTOCOL-HTTP2*; GraphQL over HTTP (working draft)

### BE-IDEMP-002

> `PUT` and `DELETE` operations MUST be idempotent in the HTTP sense: repeated identical requests MUST have the same intended effect on the resource as a single request. The response MAY differ between attempts (`201` then `200` for a `PUT` that creates, `204` then `404` for a `DELETE` retried once the resource is gone). Business side effects of the change (counter increments, notification sends, charges, outbound events) MUST NOT be repeated on `PUT`/`DELETE` retries; per-request logging and revision history MAY still record every attempt.

**Rationale:** RFC 9110 §9.2.2 defines `PUT` and `DELETE` as idempotent methods — repeated identical requests have the intended effect of one — and on that basis a client, retry middleware included, may repeat them automatically when a connection fails before the response arrives. The property covers the intended effect, not the response, and RFC 9110 leaves a server free to log each request separately, keep a revision history, or implement other non-idempotent side effects per request; audit logging (`security-backend.md` §BE-LOG) is therefore unaffected. This rule withholds that last freedom from business side effects: on a method clients retry without asking, a notification, counter increment, or charge performed per request repeats on every retry, producing duplicate-notification and double-charge incidents.

**Verification:** Any spec or plan that introduces a `PUT` or `DELETE` endpoint MUST commit to operation idempotency, including deduplication of business side effects on retries. `/{project}:analyze` flags `PUT`/`DELETE` specs that describe per-call counter increments, notification sends, charges, or outbound events without dedup.

**Source:** RFC 9110 §9.2.2

## BE-COMPAT — Backwards Compatibility

### BE-COMPAT-001

> Within a major version, schema changes MUST be additive only. Removing or renaming a field, changing a field's type (including narrowing, e.g. `string` → `enum`), changing a default value, adding a required request field or making an optional request field required, ceasing to populate a response field the server previously populated, or adding a value to a response enum the schema does not document as open to new values MUST NOT happen without a major version bump. Adding new optional request fields, new response fields, new values to enums used only in requests, new values to response enums the schema documents as open to new values, and new endpoints is permitted.

**Rationale:** Every consumer of the API has code that depends on the current shape. Removing or renaming fields breaks clients silently — the client deserializes a response and gets `null` where it expected a value, or the server rejects a request that worked yesterday. Additive evolution within a major version is the only contract that lets clients upgrade lazily. The request/response asymmetry follows the direction of data: a client never sends a value it does not know, so widening what requests accept is safe, while a client deserializing a response can fail on an enum value it has never seen or a field it was told to expect.

**Verification:** Any spec or plan that modifies an existing API endpoint MUST classify the change as additive (compatible) or breaking (requires major version). `/{project}:analyze` flags endpoint-modifying specs that omit the classification, and flags plans that describe a change listed above as breaking — subtractive, narrowing, or otherwise — within a major version without a major-version commitment.

**Source:** Semantic Versioning 2.0.0, Google AIP-180 (Backwards compatibility; the Cloud API Design Guide's compatibility page now redirects to it)

## BE-PATCH — Partial Updates

### BE-PATCH-001

> `PATCH` endpoints MUST declare their patch document format by media type — JSON Merge Patch (`application/merge-patch+json`, RFC 7396), JSON Patch (`application/json-patch+json`, RFC 6902), or a project format documented in `specs/system.md` that defines how an absent field and an explicit `null` each apply. The server MUST apply a patch atomically: when any change in the document cannot be applied, none is applied, and no partially modified representation is ever observable.

**Rationale:** A `PATCH` body means nothing without its format: under JSON Merge Patch an absent member leaves a field unchanged while `null` removes it, and an ad-hoc JSON body leaves that distinction — and with it whether a client can ever clear a field — to each handler's accident. RFC 5789 makes atomic application a requirement of the method; a patch applied halfway leaves the resource in a state no client requested and no validation saw.

**Verification:** Any spec or plan that introduces a `PATCH` endpoint MUST name its patch media type (or the documented project format) and commit to all-or-nothing application. `/{project}:analyze` flags `PATCH` specs that accept a plain `application/json` body with no stated absent-versus-`null` semantics, and flags plans that apply patch operations one by one without a transaction or equivalent all-or-nothing guard.

**Source:** RFC 5789 §2, RFC 7396, RFC 6902, Microsoft REST API Guidelines (Azure, *HTTP Return Codes* and *JSON*: `PATCH` with a JSON Merge Patch body)

## BE-COND — Conditional Requests

### BE-COND-001

> For a resource more than one client can modify, a write that replaces it, or that applies a change whose effect depends on the representation the client read, MUST let the client make the write conditional on the version it read — whether the write is a `PUT`, such a `PATCH`, or an RPC update method or GraphQL mutation. The mechanism is either HTTP conditional requests — a validator (`ETag`) on responses, `If-Match` honored on the write, and `412 Precondition Failed` on a mismatch — or a version field the schema documents (e.g., the `etag` field of AIP-154), compared by the server before applying the write, with a mismatch rejected with `409 Conflict` or, in an RPC protocol, its equivalent error (gRPC `ABORTED`).

**Rationale:** A client that reads a resource, edits it, and writes it back performs a read-modify-write across two HTTP requests. A row lock or transaction held inside the write — enough to satisfy `BE-TXN-002` within one request — cannot see that another client wrote in between, so the later write silently discards the earlier one. Only a version the client carries from its read to its write closes that window; RFC 9110 conditional requests are the standard carrier, and a documented version field is the equivalent for APIs that keep concurrency control in the body. The window does not depend on the method: an RPC update method that replaces a resource loses an update exactly as a `PUT` does, and AIP-154 gives RPC APIs the body-carried version — an `etag` field, with a mismatch failing with `ABORTED`, which it maps to HTTP `409`.

**Verification:** Any spec or plan that introduces, on a resource multiple clients can modify, a `PUT` endpoint, a `PATCH` endpoint whose patch depends on the prior representation, or an RPC update method or GraphQL mutation that replaces the resource or applies such a change MUST name its precondition mechanism and its mismatch response. `/{project}:analyze` flags such specs that commit to no precondition mechanism.

**Source:** RFC 9110 §8.8.3 (`ETag`), §13.1.1 (`If-Match`), §15.5.10 (`409` for a version conflict), §15.5.13 (`412`); RFC 5789 §2; Google AIP-154 (Resource freshness validation); Microsoft REST API Guidelines (Azure, *Conditional Requests*)

## BE-LRO — Long-Running Operations

### BE-LRO-001

> A `202 Accepted` response SHOULD identify a status monitor — a `Location` or `Operation-Location` header, or a documented field in the response body, naming a resource that reports the operation's state (pending, running, succeeded, failed). That resource SHOULD expose the result, or a link to it, on success and the failure in the project's error envelope (`BE-ERRENV-001`) on failure, and SHOULD carry `Retry-After` as the polling interval while the operation is incomplete.

**Rationale:** `performance-backend.md` `BE-ASYNC-001` requires slow work to be offloaded with a way to observe completion but leaves the shape of that way open, and RFC 9110 says a `202` representation ought to point to a status monitor. Without one contract, every asynchronous endpoint invents its own polling scheme, and a client cannot tell a job that failed from one still running — or learn why it failed. A status resource with explicit terminal states lets one client polling loop serve every asynchronous endpoint. It is advisory because a push channel (webhook, server-sent events) can serve the same purpose.

**Verification:** Any spec or plan that introduces an endpoint returning `202 Accepted` SHOULD name the status-monitor URL, the operation states it reports, and how the result and a failure are retrieved. `/{project}:analyze` emits a warning when a `202` endpoint spec leaves the completion-observation mechanism unstated or reports failure outside the project error envelope.

**Source:** RFC 9110 §15.3.3 and §10.2.3 (`Retry-After`), Microsoft REST API Guidelines (Azure, *Long-Running Operations & Jobs*), Google AIP-151 (Long-running operations)

## BE-REPR — Representation Conventions

### BE-REPR-001

> Instants (timestamps) in request and response payloads MUST be unambiguous about time zone and represented one way across the API: RFC 3339 date-time strings carrying an explicit UTC offset (`2026-10-19T14:30:00Z`, `2026-10-19T09:30:00-05:00`), or integer Unix time with its unit (seconds or milliseconds) documented in the schema, or a schema type defined as a count from the Unix epoch (protobuf `google.protobuf.Timestamp`). Date-times without an offset MUST NOT represent instants. Calendar dates and local times that denote no instant (a birthday, an opening hour) MAY use RFC 3339 `full-date` and `partial-time`.

**Rationale:** A timestamp without an offset names a different instant in every time zone that parses it, and the error is silent — values come out hours off and are never rejected. An explicit offset, or Unix time — a count from the POSIX Epoch, midnight UTC on 1 January 1970 — makes every instant parse the same everywhere; one representation across the API means one parser per client instead of one per endpoint. A `google.protobuf.Timestamp` is such a count, of seconds and nanoseconds, and the protobuf JSON mapping renders it as an RFC 3339 string with a `Z` offset.

**Verification:** Any spec or plan that introduces a timestamp field in an API request or response MUST name its representation, and `specs/system.md` MUST declare the API-wide one. `/{project}:analyze` flags timestamp fields described without a representation, with an offset-less date-time format, or with a representation that differs from the one `specs/system.md` declares.

**Source:** RFC 3339 §5.6, Microsoft REST API Guidelines (Azure, *JSON*), Google AIP-142 (Time and duration), POSIX.1-2024 Base Definitions 3.125 (*Epoch*), Protocol Buffers `timestamp.proto` and *ProtoJSON Format*

### BE-REPR-002

> Field names, query-parameter names, and enum values SHOULD each follow one casing convention (e.g., `camelCase` field names, `UPPER_SNAKE_CASE` enum values) declared in `specs/system.md` and applied across every endpoint of the API.

**Rationale:** Mixed casing (`userId` beside `created_at`) forces every client to special-case names, defeats generated-SDK naming, and makes each new endpoint a fresh style decision. A declared convention turns a reviewer's taste into a citable contract. It is advisory because a mixed-case API is inconsistent rather than incorrect.

**Verification:** Any spec or plan that introduces API fields, query parameters, or enum values SHOULD use the casing convention declared in `specs/system.md`. `/{project}:analyze` emits a warning when a spec's names mix conventions or depart from the declared one, or when a project with a public API declares none.

**Source:** Microsoft REST API Guidelines (Azure, *JSON*), Google AIP-140 (Field names)

### BE-REPR-003

> A JSON payload SHOULD NOT carry a number that a binary64 (double-precision) parser cannot hold exactly: an integer outside the range −(2^53 − 1) to 2^53 − 1, such as a 64-bit identifier, or a value with more precision than binary64 provides. Such a value SHOULD be encoded as a JSON string, documented in the schema as a string-encoded number.

**Rationale:** The JSON grammar sets no numeric limit and lets each implementation set its own, and RFC 8259 notes that integers within ±(2^53 − 1) are interoperable — implementations agree exactly on their values — while implementations built on binary64 approximate numbers beyond its range and precision. A 64-bit identifier above that range is read by such a client as a different integer, which it then displays, stores, or sends back as the wrong ID. RFC 7493 (I-JSON) recommends string encoding for exactly these values, naming 64-bit integers and the limited scope of JavaScript numbers, and the protobuf JSON mapping encodes `int64` and `uint64` fields as decimal strings. It is advisory, as RFC 7493's own guidance is.

**Verification:** Any spec or plan that introduces a JSON API field holding an integer that can exceed ±(2^53 − 1), or a value needing more precision than binary64, SHOULD encode it as a JSON string and document that encoding in the schema. `/{project}:analyze` emits a warning when a spec declares such a field — an `int64` or `uint64` identifier, a high-precision decimal — as a JSON number.

**Source:** RFC 7493 §2.2, RFC 8259 §6, Protocol Buffers *ProtoJSON Format* (`int64` and `uint64` map to a JSON string)
