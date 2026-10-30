---
section: "Two Rule Files"
---

# X-frame-options-carries-one-strength

## Context

The two security rule files stated the `X-Frame-Options` header at different strengths. `BE-API-001` in `framework/rules/security-backend.md` is MUST-tier, and its Verification listed `X-Frame-Options: DENY` (or `SAMEORIGIN`) among "the minimum required headers and their values" for HTML responses. `FE-CSP-008` in `framework/rules/security-frontend.md` stated the same header as a SHOULD: a clickjacking fallback for browsers that do not honor CSP `frame-ancestors`, which `FE-CSP-003` requires at MUST.

The disagreement predates the split that minted `FE-CSP-008`. Its clause was `FE-CSP-003`'s SHOULD half before `a-statement-carries-one-obligation-keyword` split it off. It surfaced in 008's review of that split on 2026-09-27, when both files were read in full.

A project loading both files was told the header was required and advisory at once. A missing `X-Frame-Options` was reported twice: as a blocking finding under `BE-API-001` and an advisory one under `FE-CSP-008`. In `/{project}:review` the two did not merge, because its cross-pass dedup keys on the rule ID.

## Behavior

The two rule files state one strength for `X-Frame-Options`, SHOULD, and a project that loads both is told the same thing by each.

- `BE-API-001`'s minimum-required header table no longer lists `X-Frame-Options`. Its frame protection is the CSP `frame-ancestors` directive that its CSP row already requires, and its Verification leaves a missing `X-Frame-Options` to `BE-API-012`.
- `BE-API-012` (SHOULD) states that HTML responses set `X-Frame-Options: DENY`, or `SAMEORIGIN` where same-origin framing is required, as a clickjacking fallback for browsers that do not honor `frame-ancestors`. Its Verification names `FE-CSP-008` as the frontend statement of the same obligation.
- `FE-CSP-008` keeps its SHOULD Statement and names `BE-API-012` as its backend mirror.

## Edge Cases

- **A backend-only project.** It loads `security-backend.md` without `security-frontend.md`, so the backend file has to state the header's strength itself, not leave it to the frontend rule. `BE-API-012` is where it does.
- **A project loading both files.** It gets an advisory finding under each mirror when the header is missing. Both are advisory, so neither blocks, and waiving either one silences no blocking clause.
- **Non-HTML responses.** OWASP notes the header does nothing for a redirect or a JSON API response. `BE-API-012` is scoped to HTML responses, as `BE-API-001`'s table row was, so an API-only backend gets no finding.
- **Adopter copies.** Both files ship with the `update` strategy. An unpinned copy receives the change on the next `/ductus` run, and a pinned copy keeps the disagreement until its owner acts.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Which strength does `X-Frame-Options` carry: MUST, as `BE-API-001`'s minimum-required header table says, or SHOULD, as `FE-CSP-008` says?** SHOULD, in both files. `BE-API-001`'s table stops listing `X-Frame-Options` as a minimum-required header, and a new backend rule, `BE-API-012`, states it as a SHOULD for HTML responses. It mirrors `FE-CSP-008`, so a backend-only project is still told. `FE-CSP-008` keeps its SHOULD.
  - **Why SHOULD.** The source both rules cite treats the header as an obsoleted fallback. The OWASP HTTP Headers Cheat Sheet says CSP `frame-ancestors` "obsoletes X-Frame-Options for supporting browsers" and recommends `frame-ancestors` "if possible", with `X-Frame-Options: DENY` shown alongside. The OWASP Clickjacking Defense Cheat Sheet calls the header "Deprecated", and says older browsers that ignore `frame-ancestors` may need both, as defense in depth. Both were read 2026-09-27. `BE-API-001`'s Statement asks for "frame protection", which its own CSP row already requires as `frame-ancestors` at MUST tier, so the `X-Frame-Options` row was the only thing making the legacy header blocking.
  - **Why a backend mirror rather than a SHOULD row in `BE-API-001`.** A SHOULD row inside a MUST-tier rule's Verification would bring back, at the Verification level, what `a-statement-carries-one-obligation-keyword` removed at the Statement level. A waiver or a dedup keyed on `BE-API-001` would treat the advisory header and the blocking ones as one finding. The mirror follows the precedent `BE-AUTHN-014` sets for `FE-STORAGE-002`: "a backend-only service that never loads the frontend rule set still issues these cookies, so the requirement lives here as well".
  - **Measured cost.** One rule edited (`BE-API-001`'s header table and the Rationale sentence that points at the frontend rule) and one rule added, `BE-API-012`, the next unused number in `BE-API`, whose highest ever issued in `framework/rules/security-backend.md`'s history is `011`. That makes 206 → 207 shipped rules. No spec or rule outside the two files cites `BE-API-001`. For adopters, a project loading the backend rules that omits the header moves from a blocking finding to an advisory one, and a project loading the frontend rules sees no change.
  - **Rejected.** MUST in both files would make a header its own cited source calls deprecated blocking for every HTML-serving project, including those whose targeted browsers all honor `frame-ancestors`. SHOULD in the frontend only would leave a backend-only project, which never loads `security-frontend.md`, with no rule naming the header, which this scenario's backend-only edge case rules out.
