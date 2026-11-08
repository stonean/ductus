# 068 — Backend API contract rules Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. The severity posture and the three tiers (AC12)

- [x] Add the posture sentence to the header, in the siblings' "A rule is **MUST** only when …; the rest are **SHOULD**" form, after the design-time sentence
- [x] `BE-PAGE-003`: Statement and Verification to MUST, keeping the MAY for static or append-only-at-tail collections
- [x] `BE-REPR-003`: Statement to MUST NOT and MUST, Verification to MUST and "flags"
- [x] `BE-APIVER-002`: `Sunset` no longer has to be a future date, and still must not be earlier than `Deprecation`

- **Done when**: the header states the posture as AC12 words it, the three rules read as AC12 says, `cargo test no_shipped_statement_mixes_tiers` and `scripts/lint-rule-ids.sh` pass, and AC12 is ticked.

## 2. A declared versionless strategy (AC13)

- [x] `BE-APIVER-001`: add the versionless strategy, whose `specs/system.md` declaration commits to making none of `BE-COMPAT-001`'s breaking changes, and accept it in the Verification
- [x] `BE-COMPAT-001`: a versionless API makes none of those changes, so a deprecated field or operation stays served until the API adopts a versioning strategy; the Verification flags a versionless API's spec that describes one
- [x] Add graphql.org *Schema Design* §Versioning to `BE-APIVER-001`'s Source

- **Done when**: both rules read as AC13 says, `cargo test no_shipped_statement_mixes_tiers` and `scripts/lint-rule-ids.sh` pass, and AC13 is ticked.

## 3. The API serves its own schema (AC14)

- [x] `BE-SCHEMA-001`'s Statement: served by the API itself, from a documented URL on its own origin or, for gRPC and JSON-RPC, through gRPC server reflection or the OpenRPC `rpc.discover` method, and committed to the repository
- [x] The Verification names the schema's path and how the API serves it, and keeps its *Not flagged* GraphQL clause
- [x] Add gRPC `doc/server-reflection.md` and the OpenRPC Specification's service discovery method to the Source

- **Done when**: `BE-SCHEMA-001` reads as AC14 says, `cargo test no_shipped_statement_mixes_tiers` and `scripts/lint-rule-ids.sh` pass, and AC14 is ticked.

## 4. `ERRENV` governs a resource-oriented HTTP API (AC15)

- [x] The header's `ERRENV` declaration and reach sentence limit the category to a resource-oriented HTTP API, an RPC protocol carried over HTTP counting as an API that is not one, whose errors are `security-backend.md` `BE-ERR-002`'s and `BE-ERR-004`'s
- [x] Each of `BE-ERRENV-001`, `BE-ERRENV-002` and `BE-ERRENV-003` names a resource-oriented HTTP API as its subject in its Statement
- [x] Confirm `BE-ERR-002`'s and `BE-ERR-004`'s Verification still reach such an API, so `security-backend.md` needs no edit

- **Done when**: the header and the three Statements read as AC15 says, `cargo test no_shipped_statement_mixes_tiers` and `scripts/lint-rule-ids.sh` pass, and AC15 is ticked.

## 5. A deprecated gRPC method's headers (AC16)

- [ ] `BE-APIVER-002`'s Statement: a deprecated gRPC method carries `Deprecation` and `Sunset` as response metadata, which gRPC sends in its HTTP/2 response headers
- [ ] Add gRPC *PROTOCOL-HTTP2* (Response-Headers, Custom-Metadata) to the Source

- **Done when**: `BE-APIVER-002` reads as AC16 says, `cargo test no_shipped_statement_mixes_tiers` and `scripts/lint-rule-ids.sh` pass, and AC16 is ticked.

## 6. Re-fetch every Source (AC4)

The set is every document a Source line cites after tasks 1–5. As read at `719c7a19`, with the two documents tasks 2 and 3 add:

- IETF: RFC 3339, 5789, 6585, 6902, 7396, 7493, 8259, 8594, 9110, 9457, 9651 and 9745, and `draft-ietf-httpapi-idempotency-key-header-07`
- Google AIP-140, 142, 151, 154, 155, 180, 185 and 193
- Microsoft REST API Guidelines (Azure): *API Versioning*, *HTTP Return Codes*, *JSON*, *Considerations for sorting with pagination*, *Conditional Requests*, *Long-Running Operations & Jobs*
- The OpenAPI Specification (Operation Object) and the OpenAPI Initiative's best practices
- The GraphQL Specification (September 2025) §3 and §3.13.3, the GraphQL Cursor Connections Specification, GraphQL over HTTP (working draft), and graphql.org *Schema Design*
- gRPC *Introduction to gRPC*, *PROTOCOL-HTTP2* and `doc/server-reflection.md`, and Protocol Buffers `descriptor.proto` and `timestamp.proto`
- The OpenRPC Specification and the JSON-RPC 2.0 Specification
- Stripe's error, pagination and idempotency documentation
- PostgreSQL's *LIMIT and OFFSET*, Semantic Versioning 2.0.0, and POSIX.1-2024 Base Definitions 3.125

Reuse the eight copies the plan names, from the clarify session's scratchpad `068-fetch/`. Fetch the rest into this session's scratchpad `068-fetch/`, never committed, with `curl` and `scripts/check-runtime-currency.sh`'s User-Agent. Use WebFetch for verbatim quotes where `curl` is refused. Rewrite or remove a claim no fetched copy confirms.

- [ ] Fetch the set, reusing the eight copies
- [ ] Check each claim of each rule against its fetched copy, correcting a claim or a section reference the copy does not support
- [ ] The landing commit's message names each document, its URL, the fetch date and the method, WebFetch beside each claim it confirmed

- **Done when**: every claim matches a fetched copy or was rewritten or removed, `cargo test no_shipped_statement_mixes_tiers` and `scripts/lint-rule-ids.sh` pass, and AC4 is ticked.

## 7. Verify the remaining criteria (AC1–AC3, AC5–AC11)

Blocked on task 6, which can rewrite rule text. Once every box in this file is ticked, `/ductus:review` runs, then `/ductus:analyze`, then the completion gate with the owner's approval.

- [ ] AC1: the header's categories against the scope table, and `scripts/lint-rule-ids.sh`
- [ ] AC2: categories disjoint from the five other `-backend.md` headers
- [ ] AC3: each rule's heading, Statement and Verification, and no Rationale
- [ ] AC5, AC6, AC8 and AC9: each rule read against the spec
- [ ] AC7: `check-rule-ids`, each cited rule's text, and every citation into this file from the other rule files
- [ ] AC10: the manifest row in `framework/bootstrap/ductus.md`, and `discover-rule-files` under `backend`
- [ ] AC11: `framework/constitution.md` names no `api-backend.md`

- **Done when**: each criterion holds against the tree, with a failure fixed in this task, and AC1–AC3 and AC5–AC11 are ticked.
