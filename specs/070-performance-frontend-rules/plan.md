# 070 — Performance rules (frontend) Plan

Implements [070 — Performance rules (frontend)](spec.md).

## Overview

070 takes ownership of a rule file that already ships, so most of its criteria describe `framework/rules/performance-frontend.md` as it stands and are verified against the tree, not built. Its edits to the file are the two 2026-11-08 decisions in Resolved Questions, one task each. AC11 adds the header's severity posture and moves five rules to SHOULD. AC12 has `FE-VITALS-001` publish its budgets in `specs/system.md`. After them comes the re-fetch of every document a Source line cites (AC3's claims, AC4), so the edited text is checked too. A last walk verifies AC1, AC2, AC3's schema and AC5–AC10 against the edited file. The rule file is the only deliverable that changes. The `FE` surface and its ID format already exist, so `scripts/lint-rule-ids.sh` is unchanged, and the spec's *ID format* section rules out a data model.

## Technical Decisions

### The header states the severity posture (AC11)

The header gains the posture sentence from Resolved Questions in the form its siblings use. That form is "A rule is **MUST** only when …; the rest are **SHOULD**" (line 3 of `framework/rules/concurrency-backend.md`, `performance-backend.md`, `reliability-backend.md` and `api-backend.md`). The sentence: a rule is MUST only when its absence blocks a user regardless of scale, such as text left invisible, rendering held by a parser-blocking script, or the main content's image deferred, or leaves the page without the budgets and real-user measurement the other rules answer to, and SHOULD where it is one means among others toward a budget. It sits after the design-time sentence, and the header stays one paragraph.

### Five rules move to SHOULD under their IDs (AC11)

Each of `FE-BUNDLE-002`, `FE-IMAGE-001`, `FE-IMAGE-002`, `FE-LOAD-001` and `FE-FONT-002` takes one tier throughout. Its Statement's MUST becomes SHOULD and its MUST NOT becomes SHOULD NOT. `FE-LOAD-001`'s "MUST be code-split per `FE-BUNDLE-002`" moves with it. Each Verification moves from "MUST commit" to "SHOULD commit" and from "flags" to "emits a warning when", the form the file's SHOULD rules use (`FE-IMAGE-003`, `FE-LOAD-003`). Each keeps its **Not flagged** clause and its yields. The six that stay MUST are `FE-VITALS-001`, `FE-VITALS-002`, `FE-BUNDLE-001`, `FE-LOAD-002`, `FE-LOAD-005` and `FE-FONT-001`. None of them names any of the five, so no MUST case yields to a rule that is now SHOULD (a script over each rule's section at `4c25aeb6`). `FE-LOAD-001` yields the initial viewport to `FE-LOAD-005`, which stays MUST, and `FE-LOAD-008` yields an undimensioned `<img>` to `FE-IMAGE-002`, both now SHOULD.

The file holds 22 rules, 11 MUST and 11 SHOULD, at `4c25aeb6`, counted from each `### FE-` section's Statement by its first keyword. After this decision it holds 6 MUST and 16 SHOULD, and the spec's *Severity posture* says so. A tier change is proved by `no_shipped_statement_mixes_tiers` (`runtime/src/primitives/rule_sections.rs`), which every task that edits a Statement runs.

No other artifact states these rules' tiers. Outside this spec and the rule file, `git grep` for the five IDs finds only 008's scenario `a-statement-carries-one-obligation-keyword`. It lists `FE-FONT-002` and `FE-LOAD-001` in a count dated 2026-09-27, which stays true of that date. No `review.md` or `analysis.md` stores a decision naming any of the five.

### `FE-VITALS-001` publishes its budgets in `specs/system.md` (AC12)

The Statement's "MUST publish documented performance budgets" names `specs/system.md` as their home, as `FE-BUNDLE-001`'s does. The sentence on looser budgets keeps its pointer there. The Verification adds a flag for a frontend project with no Web Vitals budgets published in `specs/system.md`, mirroring `FE-BUNDLE-001`'s "flags frontend projects with no budgets published in `specs/system.md`". The spec's `VITALS` row names the home.

### Every Source is re-fetched after the edits (AC3, AC4)

The set is every document a Source line, or a claim in any field, cites once tasks 1–2 land. Task 3 names it, about 30 documents: web.dev articles, MDN pages, RFC 9111 and RFC 8246, W3C CSP Level 3 and the Long Tasks API, the Baseline (web-features) status data, the framework routing references, Chrome for Developers, Filament Group, Lighthouse's DOM-size insight, Addy Osmani's article, and the `web-vitals` library documentation. The method is 068's task 6 (`a8f419f7`) and 069's task 4 (`12b26fab`):

- Fetching uses `curl` with `scripts/check-runtime-currency.sh`'s repository-URL User-Agent, into the implementing session's scratchpad, never committed.
- A page whose content its HTML does not carry, because it renders client-side, is read from its documentation source repository's raw file. The commit names that method beside the claim.
- Read-only agents check rule groups against the copies, and every quote behind an edit is confirmed present by script before the edit.
- A claim no copy confirms is rewritten to what the copy says, or removed, never kept on recall. A Source naming a section its document lacks is corrected.
- The landing commit's message is the provenance: each document, its URL, the fetch date and the method.

AC4's thresholds and measurement basis are checked against web.dev's "Defining the Core Web Vitals metrics thresholds" and "Web Vitals". The fetches are independent and may run in parallel agents, each writing only its own copies, while edits to the rule file stay serial.

### The remaining criteria are verified last (AC1, AC2, AC3's schema, AC5–AC10)

Task 3 can rewrite rule text, so the walk follows it. The method for each criterion:

- **AC1, AC2**: the header against the scope table. The `## FE-` sections of `security-frontend.md` and `accessibility-frontend.md` give the other categories. Then `scripts/lint-rule-ids.sh`.
- **AC3's schema**: the corpus test, plus a read of each rule's heading, Statement, Verification and Source.
- **AC5, AC6**: a read of each MUST Statement and Verification, and of `FE-BUNDLE-001`.
- **AC7, AC8**: `check-rule-ids` against every file in `framework/rules/`, a read of each cited rule, and `BE-CACHE-004`'s deferral in `performance-backend.md`.
- **AC9**: the manifest row (`framework/bootstrap/ductus.md:875`), plus `discover-rule-files` under `frontend`.
- **AC10**: a grep of `framework/constitution.md`.

A check that fails is fixed inside that task, since it is inside this spec.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/rules/performance-frontend.md` | Modify | The header's posture; five rules to SHOULD; `FE-VITALS-001`'s budgets in `specs/system.md`; Source corrections from the re-fetch |
| `specs/070-performance-frontend-rules/spec.md` | Modify | *Severity posture*'s counts, the `VITALS` row, and the ticked criteria |

A change under `framework/rules/` runs `runtime.yml`, whose `paths` list `framework/rules/**` and whose corpus test reads every shipped Statement, and `framework-checks.yml`, whose self-audit resolves every `§` reference in a shipped file. So each task runs the tier test, `scripts/lint-rule-ids.sh`, markdownlint and `scripts/audit/run-all.sh` before it is pushed, and the whole `cargo test` runs once before the push. No golden moves with a rule-file edit: the parity fixtures hold their own `framework/rules/`.

## Trade-offs

- **`FE-IMAGE-001`'s fallback moves to SHOULD with the rest of it (decided by the owner's delegation, 2026-11-08).** The Statement names two mechanisms, and each gives a browser something it can render. `<picture>` has a required `<img>` child, whose content model is "Zero or more source elements, followed by one img element" (HTML Living Standard §4.8.1, fetched 2026-11-08). Server or CDN negotiation on `Accept` serves a format the request accepts. Only a bare modern-format `<img>` loses the fallback, and the Statement never endorses one. The posture is a necessary condition for MUST, not a sufficient one, so a SHOULD clause whose absence can block a user does not contradict the header. Rejected: splitting the fallback into a new MUST rule. It would add a requirement the owner's decision does not name. And 008's split convention gives the MUST clauses the existing ID, while AC11 keeps `FE-IMAGE-001` SHOULD under its ID, so a split would break one of them.
- **One task per decision, not one pass over the file.** Each tier change runs the corpus test on its own, and each commit says which decision it applies.
- **The re-fetch after the edits, not before.** Fetching first would leave the edited text unchecked, or need a second fetch.
- **No data model**, per the spec's *ID format*: one would compete with the header as the source of the file's categories.
- **Known limitation.** AC3 and AC4 check each claim against its document as fetched on one date. A later revision of a cited document drifts unseen, since no check watches rule Sources. Like every rule file's, this file's Verification proves that a spec commits to a contract, not that code honors it.

## Cross-spec impact

The spec body links 008, 011, 024, 025, 033, 034, 037 and 050. None needs an edit. `performance-backend.md`'s `BE-CACHE-004` already defers build-emitted frontend assets to `FE-LOAD-007` where this file is loaded, and AC7 checks it. No other rule file cites the five rules whose tier changes. 008's scenario records its 2026-09-27 count. No stored decision names any of the five. So `cross-spec-impact:` stays empty.
