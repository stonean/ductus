# 068 — Backend API contract rules Plan

Implements [068 — Backend API contract rules](spec.md).

## Overview

068 takes ownership of a rule file that already ships, so most of its criteria describe `framework/rules/api-backend.md` as it stands and are verified against the tree, not built. Its edits to the file are the five 2026-11-08 decisions in Resolved Questions (AC12–AC16), one task each. After them comes AC4's re-fetch of every document a Source line cites, so the claims the decisions add are checked too. A last walk then verifies AC1–AC3 and AC5–AC11 against the edited file. The rule file is the only deliverable that changes. The `BE` surface and its ID format already exist, so `scripts/lint-rule-ids.sh` is unchanged, and the spec's *Rule ID format* section rules out a data model. AC11 needs no constitution edit, since the owner dropped its pointer half (`719c7a19`).

## Technical Decisions

### The header states the severity posture and the `ERRENV` reach (AC12, AC15)

The header gains the posture sentence from the spec's *Severity posture*, in the form its siblings use: "A rule is **MUST** only when …; the rest are **SHOULD**" (`framework/rules/concurrency-backend.md`, `performance-backend.md` and `reliability-backend.md`, line 3 of each). It sits after the design-time sentence, and the header stays one paragraph. The `ERRENV` entry in the category declaration says the category governs a resource-oriented HTTP API. The reach sentence says an RPC protocol carried over HTTP counts, for `ERRENV`, as an API that is not an HTTP API, whose errors are `security-backend.md` `BE-ERR-002`'s and `BE-ERR-004`'s. Each `ERRENV` Statement names a resource-oriented HTTP API as its subject, which AC15 asks of the Statements themselves. `security-backend.md` does not change: `BE-ERR-002`'s Verification already reaches "an HTTP API or one that is not, such as a gRPC or JSON-RPC service" (`framework/rules/security-backend.md:637`), and `BE-ERR-004`'s Verification reaches "an API that is not an HTTP API" (`:653`).

### Three rules take the tier the posture gives them (AC12)

- `BE-PAGE-003` becomes MUST: cursor-based pagination for a collection under concurrent inserts or deletes. Its MAY for static or append-only-at-tail collections stays, and its Verification moves from "emits a warning" to "flags".
- `BE-REPR-003` becomes MUST NOT and MUST, with its Verification moved the same way. Its Source still cites RFC 7493 §2.2 for the mechanism, not for the tier (Resolved Questions).
- `BE-APIVER-002`'s `Sunset` keeps its MUST but drops "a future". RFC 8594 §3 says a past date is read as the present time, so a past date means removal at any time.

No other artifact states these rules' tiers. Outside this spec and the rule file, a grep at `719c7a19` over `framework/`, `specs/`, `docs/`, `runtime/src` and `runtime/tests` finds two records. The grep looked for the three IDs, for `cursor-based` and for `binary64`.

- 008's scenario `a-statement-carries-one-obligation-keyword` names "the SHOULD rule" that `BE-PAGE-001`'s 2026-09-27 split produced. That is a record of the split, and it stays true of it.
- 063's `analysis.md` stores the owner's `discarded` decision on a `BE-REPR-003` finding (2026-10-20). A person's stored decision wins whatever the tier, so that record needs no edit.

A tier change is proved by `no_shipped_statement_mixes_tiers` (`runtime/src/primitives/rule_sections.rs:449`), which every task that edits a Statement runs.

### A declared versionless strategy (AC13)

`BE-APIVER-001`'s list of strategies adds a versionless one. Its declaration in `specs/system.md` commits to making none of the changes `BE-COMPAT-001` lists as breaking for as long as the API stays versionless. `BE-COMPAT-001` adds that such an API makes none of those changes, so a deprecated field or operation stays served until the API adopts a versioning strategy. Its Verification flags a versionless API's spec that describes one. `BE-APIVER-001` stays MUST, and its Source adds graphql.org *Schema Design* (*Versioning*). `BE-APIVER-002` sends no `Sunset` on such an API's deprecated endpoint, which stays served, as RFC 8594 §1.4 has `Sunset` "not appropriate" while an API "remains operational" (Resolved Questions).

### The API serves its own schema (AC14)

`BE-SCHEMA-001`'s second sentence requires the schema to be served by the API itself: from a documented URL on its own origin, or, for gRPC and JSON-RPC, through the protocol's discovery mechanism (gRPC server reflection, the OpenRPC `rpc.discover` method). The schema must also be committed to the repository. The Verification names the schema's path and how the API serves it, and its *Not flagged* GraphQL clause stays. The Source adds gRPC `doc/server-reflection.md` and the OpenRPC Specification's service discovery method. Eleven specs' review records store a decision keyed on "BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002": 017, 019, 022, 025, 034, 036, 037, 038, 039, 050 and 060. Each spec's own next review matches or expires that key, and 068 edits none of their records.

### A deprecated gRPC method carries the two headers as metadata (AC16)

`BE-APIVER-002`'s Statement says a deprecated gRPC method sends the headers the rule requires of it as response metadata, named `deprecation` and `sunset` since gRPC metadata names are lowercase, which gRPC carries in the HTTP/2 response headers. Its schema mark stays protobuf `option deprecated = true`. The Source adds gRPC *gRPC over HTTP2* (`doc/PROTOCOL-HTTP2.md`), and the header's reach list already names `BE-APIVER-002`.

### Every Source is re-fetched after the edits (AC4)

The set is every document a Source line cites once tasks 1–5 land. Task 6 names it, as read at `719c7a19` plus the two documents tasks 2 and 3 add. Eight copies fetched on 2026-11-08 for clarify are reused: RFC 8594, RFC 7493, RFC 9745, the Idempotency-Key draft-07, graphql.org *Schema Design*, gRPC `server-reflection.md` and `PROTOCOL-HTTP2.md`, and the OpenRPC Specification. Copies live in the implementing session's scratchpad under `068-fetch/` and are never committed. The landing commit's message is the provenance: each document, its URL, the fetch date and the method. Fetching uses `curl` with `scripts/check-runtime-currency.sh`'s repository-URL User-Agent. A host that refuses `curl` is read with WebFetch for verbatim quotes, and the commit names that method beside the claim. A claim that no fetched copy confirms is rewritten to what the copy says, or removed, and never kept on recall. A Source naming a section its document lacks is corrected. The fetches are independent and may run in parallel agents, each writing only its own copies; edits to the rule file stay serial.

### The remaining criteria are verified last (AC1–AC3, AC5–AC11)

AC4 can rewrite rule text, so the walk follows it. The method for each criterion:

- **AC1**: the header against the scope table, plus `scripts/lint-rule-ids.sh`.
- **AC2**: the five other `-backend.md` headers' categories (`RACE`/`LOCK`/`TXN`/`COORD`, `METRIC`/`TRACE`/`HEALTH`, `QUERY`/`CACHE`/`POOL`/`PAYLOAD`/`ASYNC`, `TIMEOUT`/`RETRY`/`BREAKER`/`DRAIN`/`BULK`, `AUTHN`/`AUTHZ`/`INPUT`/`DATA`/`API`/`LOG`/`DEPS`/`ERR`), read at `719c7a19`.
- **AC3**: the corpus test, plus a read of each rule's heading, Statement and Verification.
- **AC5, AC6, AC8, AC9**: a read of each rule against the spec.
- **AC7**: `check-rule-ids`, plus a read of each cited rule and of every other rule file's citations into this one.
- **AC10**: the manifest row (`framework/bootstrap/ductus.md:870`), plus `discover-rule-files` under `backend`.
- **AC11**: a grep of `framework/constitution.md`.

A check that fails is fixed inside that task, since it is inside this spec.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/rules/api-backend.md` | Modify | The posture and `ERRENV` reach in the header; three tiers; versionless; the self-served schema; gRPC deprecation metadata; Source corrections from the re-fetch |
| `specs/068-api-backend-rules/spec.md` | Modify | AC11's wording (`719c7a19`) and the ticked criteria |

A change under `framework/rules/` runs `runtime.yml`, whose corpus test reads every shipped Statement, and `framework-checks.yml`.

## Trade-offs

- **AC11's pointer, rejected by the owner (`719c7a19`).** Pointing §rules at this spec would cost 50 bytes against the constitution's 12 bytes of headroom. It would also name one shipped file's spec, the enumeration 021's Resolved Questions keep out of the constitution, and it resolves only in this repository.
- **One task per decision, not one pass over the file.** Each tier change runs the corpus test alone, and each commit says which decision it applies.
- **The re-fetch after the edits, not before.** Fetching first would leave the claims the decisions add unchecked, or need a second fetch.
- **No data model**, per the spec's *Rule ID format*: one would become the first-tier source for the file's ID format and would have to be kept complete.
- **Known limitation.** AC4 checks each claim against its document as fetched on one date. A later revision of a cited document drifts unseen, since no check watches rule Sources. Like every rule file's, this file's Verification proves that a spec commits to a contract, not that code honors it.

## Cross-spec impact

The spec body links 008, 016, 024, 033, 034, 038, 039 and 050. None needs an edit. `security-backend.md`'s `BE-ERR-002` and `BE-ERR-004` already reach an API that is not an HTTP API. No other rule file cites the changed tiers. 008's scenario records its split. 063's stored decision is a person's. AC11 holds without a constitution edit. So `cross-spec-impact:` stays empty.
