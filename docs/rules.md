# Rules

The deep reference for the rules `/review` holds code to. The README's [Rules](../README.md#rules) section covers what they are and what ships; this page covers how a rule is enforced, which rule files load for your project, how waivers work, and how to write your own.

## How a rule is enforced

A spec says what a feature should do; the rules say what any code is held to regardless of feature — and `/review` audits the implementation against them before a spec can reach `done`. Without them the review has taste and nothing else.

They are plain markdown in your project's rule-file directory — the `rules/` directory under your spec root once scaffolded, or [`framework/rules/`](../framework/rules/) here in `ductus`'s own repo — and they are the **only** normative source a review may cite: `/review` is instructed not to invent criteria beyond these files and your `AGENTS.md`. That is what keeps two reviews of the same code from disagreeing, and what makes a finding arguable — every one quotes the rule it came from.

**Every rule uses RFC 2119 language, and the distinction is load-bearing.** **MUST** / **MUST NOT** violations are blocking: they hold the spec out of `done` until they are fixed or waived with a recorded reason. **SHOULD** / **SHOULD NOT** violations are advisory — reported, never blocking the review record — but an outstanding SHOULD still keeps the spec out of `done` until it is fixed or waived.

**Every rule carries a permanent ID** (`BE-AUTHN-003`, `FE-XSS-001`, `QUAL-CLAIM-001`). IDs are never renumbered or reused, even when a rule moves within its file, so a waiver, a code comment, or a spec can cite one and still mean the same thing years later.

## What ships

| Rule file | Covers |
| --- | --- |
| `security-backend.md` | Authentication, authorization, input validation, data protection, API security, logging and audit, dependency management, error handling |
| `security-frontend.md` | XSS, CSRF, secure client-side storage, authentication UX, content security policy, dependencies, handling of sensitive data |
| `quality-cross.md` | Silent stubs, unverified contracts, unsubstantiated clean results, unenumerated delegation to shared code, tests that cannot fail |
| `reliability-backend.md` | Bounded timeouts and deadlines, retry discipline and dead-lettering, circuit breakers, graceful shutdown, bulkheads that shed load rather than queue unboundedly, graceful degradation when a non-critical dependency fails |
| `performance-backend.md` | Query efficiency (N+1, unindexed filters, unbounded reads, batched writes), caching with a stated expiry, connection-pool discipline, payload budgets and streaming, offloading slow work off the request path |
| `performance-frontend.md` | Core Web Vitals budgets, bundle size, image delivery, resource loading (prioritization, caching, layout reservation), web-font discipline, main-thread work |
| `api-backend.md` | The *contract* other people call: schema publication, versioning and deprecation, error envelopes, status-code semantics, pagination, idempotency, breaking-change discipline, partial updates, conditional requests, long-running operations, representation conventions (the HTTP *attack* surface is `security-backend.md` §BE-API) |
| `concurrency-backend.md` | Shared-state races, locking and deadlock avoidance, transaction isolation, distributed coordination |
| `observability-backend.md` | Metrics, distributed tracing, health signaling — so a failure in production is diagnosable rather than merely visible |
| `accessibility-frontend.md` | WCAG 2.2 AA: semantic HTML, keyboard navigation and focus, ARIA usage, color and contrast, accessible forms, text alternatives and media, pointer input and target size, reflow and zoom, text spacing and orientation, motion and flashing, time limits, accessibility testing |
| `configuration-cross.md` | Named constants, environment variables, and feature flags: no magic numbers, no undocumented env vars, fail-fast on missing or malformed configuration, no secrets baked into client builds, a declared default for every flag |

The security files are mostly **MUST** rules, because the failures they describe are exploitable rather than merely untidy. The backend reliability, observability, performance, and concurrency files default to **SHOULD** where the right approach depends on context, and each states in its header what earns a rule **MUST**. Accessibility is AA because that is the baseline written into law in several jurisdictions, which is why it is a shipped rule file rather than a suggestion; a rule that adopts a Level AAA criterion says so and is a **SHOULD**. `quality-cross.md`'s five failure modes are stated in full in the README's [Rules](../README.md#rules) section.

## Which rules load

A rule file's **filename suffix** decides which projects load it, and there are exactly three:

| Suffix | Loads for | Example |
| --- | --- | --- |
| `-backend.md` | projects with a backend surface | `security-backend.md` |
| `-frontend.md` | projects with a frontend surface | `security-frontend.md` |
| `-cross.md` | every project, unconditionally | `quality-cross.md` |

Set `[rules] surfaces` in `.ductus/config.toml` to declare your surfaces (`["backend"]`, `["backend", "frontend"]`, or `[]` for cross-only); leave it unset and `ductus` falls back to the stack it detects. A file with an unrecognized suffix loads for *every* stack and warns — the default is never a silent skip. Suffix and `surfaces` decide what `ductus` installs and what `/review` enforces; `/analyze` loads every rule file present, and a rule fires there only when a spec or plan meets its Verification trigger.

To stand a rule file down from `/review` without deleting it — a `-cross.md` file included — list it under `[[review.disabled-rule-files]]` with a mandatory `reason`. The reason is the audit trail, and it is why the opt-out is a config entry rather than a deletion.

## Waivers

When a **MUST** violation is intentional, record a waiver rather than silencing the gate:

```text
/review --waive <rule-id> --reason "<text>"
```

The waiver is anchored to the `(rule, file)` pair, so code moving within a file keeps it while a rename or a fix expires it and the finding re-blocks — a waiver cannot quietly outlive the thing it excused. When the rule fires in several files, one waiver lists them all, and each listed path still expires on its own. The schema is open, so an organization can require its own fields (a ticket, a second approver) and `ductus` preserves them. See [specs/020-code-review/data-model.md](../specs/020-code-review/data-model.md).

## Writing your own

Add a file to the rule-file directory — `rules/` under your spec root, or [`framework/rules/`](../framework/rules/) here — with one of the three suffixes and it is discovered automatically — no registration step, no manifest entry. Give each rule a permanent ID, state it in RFC 2119 language, and record the rationale: a rule whose reasoning is not written down is one nobody can argue with when it is wrong.

**A rule file you wrote is yours.** `/ductus` only ever rewrites the rule files it ships; it does not touch, move, or delete a file you added, on any upgrade. The one case that needs a decision is a shipped file you have *edited* — that one is overwritten on the next update unless you pin it in `.ductus/config.toml` `[pinned] files`. Upstreaming the change is usually better than pinning: a pinned file stops receiving fixes.

A rule true for your organization's projects rather than one project's — a house code style, a deployment gate — can live in a [shared constitution](shared-constitution.md) instead, registered once and loaded by every project.
