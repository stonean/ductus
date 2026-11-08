# Security Rules — Frontend

Security rules for browser-side code, UI rendering, and client-server interaction, applying to every project with a web frontend. Each rule's Verification states a **design-time commitment** — what a spec or plan must state — which `/{project}:analyze` checks against feature artifacts; when `/{project}:review` selects this file, its security pass also checks code in scope against each rule's Statement. Rule IDs follow the format `FE-{CATEGORY}-{NNN}`. Categories: `XSS` (cross-site scripting), `CSRF` (cross-site request forgery), `STORAGE` (secure client-side storage), `AUTHN` (authentication UX), `CSP` (content security policy), `DEPS` (dependency management), `PII` (sensitive data handling). Severity, ID permanence, and how review selects or excludes this file follow constitution §rules.

## FE-XSS — Cross-Site Scripting Prevention

### FE-XSS-001

> All untrusted data rendered in HTML MUST be encoded for the specific output context (HTML body, HTML attribute, JavaScript, CSS, URL); a single encoding strategy MUST NOT be reused across contexts.

**Verification:** Any spec or plan that introduces user-rendered output (search keywords: `render`, `template`, `interpolate`, `display`, `html`) MUST name the output context for each rendered value and the matching encoding strategy. `/{project}:analyze` flags rendering specs that describe a single global encode/escape step with no context distinction, and specs that bind untrusted data into JavaScript, CSS, or URL contexts without naming a context-specific encoder.

**Source:** OWASP XSS Prevention Cheat Sheet

### FE-XSS-002

> When the application uses a UI framework with automatic output encoding (React JSX, Angular templates, Vue templates), it MUST rely on the framework's auto-escaping as the primary XSS defense; explicit escape hatches (`dangerouslySetInnerHTML`, `bypassSecurityTrustHtml`, `v-html`) MUST be justified in the spec or plan and accompanied by a sanitization commitment.

**Verification:** Any spec or plan that describes UI rendering with a framework MUST commit to auto-escaping by default and MUST identify any planned escape-hatch usage with the sanitization strategy (e.g., DOMPurify) for that location. `/{project}:analyze` flags rendering specs that name an escape-hatch API — the Statement's, or an `innerHTML`-binding directive — without an accompanying sanitization commitment.

**Source:** OWASP XSS Prevention Cheat Sheet

### FE-XSS-003

> When manipulating the DOM with untrusted data, code MUST use safe assignment APIs (`.textContent`, `.setAttribute()` with allowlisted attributes, `element.value`) over unsafe alternatives (`.innerHTML`, `.outerHTML`, `eval()`, `document.write()`, `new Function(...)`); when `.innerHTML` is unavoidable, content MUST first be passed through a dedicated sanitization library (DOMPurify or equivalent).

**Verification:** Any spec or plan that describes DOM manipulation, dynamic content insertion, or client-side rendering of fetched data MUST commit to safe-API usage as the default. `/{project}:analyze` flags client-rendering specs that name `outerHTML`, `eval`, `document.write`, or `Function(...)` for untrusted-data sinks, and those that name `innerHTML` for them without an accompanying sanitization commitment.

**Source:** OWASP DOM-based XSS Prevention Cheat Sheet

### FE-XSS-004

> Inline `<script>` blocks and inline event-handler attributes (`onclick`, `onerror`, `onload`, etc.) SHOULD NOT be used; JavaScript SHOULD be loaded from external files or inlined behind a CSP nonce/hash.

**Verification:** Any spec or plan that describes UI rendering or HTML response composition SHOULD commit to externalizing scripts and event handlers. `/{project}:analyze` emits a warning when a frontend-rendering spec proposes inline scripts or HTML-attribute event handlers without naming a CSP nonce/hash strategy.

**Source:** OWASP CSP Cheat Sheet, MDN CSP documentation

### FE-XSS-005

> When the application renders user-authored markup (rich-text editors, markdown preview, comment systems with HTML), the rendered output MUST be passed through a dedicated HTML sanitization library configured with an explicit allowlist of tags and attributes; the sanitized output MUST NOT be modified after sanitization.

**Verification:** Any spec or plan that allows user-authored HTML, markdown rendering, or rich-text content MUST name the sanitization library (DOMPurify, sanitize-html, etc.) and the configured tag/attribute allowlist. `/{project}:analyze` flags user-content specs that omit the sanitizer name, that propose hand-rolled regex sanitization, or that describe modifying the sanitizer's output before it reaches the DOM. **Not flagged:** modified output that is passed through the sanitizer again before it reaches the DOM.

**Source:** OWASP XSS Prevention Cheat Sheet

### FE-XSS-006

> Untrusted URLs bound to `href`, `src`, `action`, `formaction`, or redirect targets MUST be validated against an explicit allowlist of acceptable schemes. The allowlist MUST NOT include `javascript:`, `data:`, `vbscript:`, or `blob:` for any URL whose value is influenced by user input. (`data:` and `blob:` may appear in narrowly scoped contexts where the URL is produced by application code over content the application controls, but never directly from user input.)

**Verification:** Any spec or plan that binds user-controlled values into URL-bearing attributes MUST commit to a project-defined scheme allowlist (e.g., `https:`, `mailto:`, `tel:`). `/{project}:analyze` flags link-rendering, image-rendering, and redirect-handling specs that omit the scheme-allowlist commitment, and any allowlist that includes a scheme the Statement excludes for user-influenced URLs.

**Source:** OWASP XSS Prevention Cheat Sheet, OWASP XSS Filter Evasion Cheat Sheet, W3C Content Security Policy Level 3, MDN `data:` URLs and `blob:` URLs documentation, Microsoft Edge Blog "An update on disabling VBScript in Internet Explorer 11"

### FE-XSS-007

> Code that receives `window.postMessage` events MUST validate `event.origin` against an explicit allowlist before acting on `event.data`, and MUST treat `event.data` as untrusted input subject to the same validation, sanitization, and context-aware encoding rules as any other external input.

**Verification:** Any spec or plan that introduces `postMessage` consumers (iframe embeds, OAuth popup callbacks, widget bridges) MUST commit to (a) origin-allowlist validation and (b) data validation/encoding before use. `/{project}:analyze` flags client-messaging specs that omit either check or that rely on wildcard origin checks. **Not flagged:** a navigation target taken from `event.data` and not checked against an allowlist of destinations, which `FE-AUTHN-001` owns; its scheme stays `FE-XSS-006`'s.

**Source:** OWASP HTML5 Security Cheat Sheet, MDN postMessage documentation

### FE-XSS-008

> When an application adopts the Trusted Types API, its CSP MUST include `require-trusted-types-for 'script'` and a `trusted-types` directive naming the allowed policy names, and sinks that accept strings (`innerHTML`, `outerHTML`, `document.write`, `eval`, `Function`, script `src`) MUST receive `TrustedHTML`/`TrustedScript`/`TrustedScriptURL` values produced by a reviewed policy, not raw strings. A reviewed policy named `default` MAY receive the raw strings of legacy code paths while they are migrated to explicit policies.

**Verification:** Any spec or plan that adopts Trusted Types MUST commit to the Statement's two CSP directives, a policy-naming convention, and routing string-accepting sinks through reviewed policies. `/{project}:analyze` flags specs that adopt Trusted Types without naming both directives, or that describe string-accepting sinks receiving raw strings under an adoption with no reviewed `default` policy to route them through.

**Source:** W3C Trusted Types specification, MDN Trusted Types API documentation, OWASP XSS Prevention Cheat Sheet

### FE-XSS-009

> Applications that render dynamic HTML in the browser SHOULD adopt the Trusted Types API.

**Verification:** Any spec or plan covering rendering of dynamic HTML in the browser SHOULD commit to adopting Trusted Types. `/{project}:analyze` emits a warning when a frontend rendering spec omits the adoption discussion. This rule governs whether Trusted Types is adopted: an adoption missing its CSP directives or feeding raw strings to sinks is `FE-XSS-008`'s.

**Source:** W3C Trusted Types specification, MDN Trusted Types API documentation

### FE-XSS-010

> Code that sends a `window.postMessage` message carrying non-public data (tokens, authorization codes, user data, application state) MUST pass the exact origin of the intended recipient as the `targetOrigin` argument; `'*'` MUST NOT be used as the `targetOrigin` of such a message.

**Verification:** Any spec or plan that introduces `postMessage` senders (OAuth or payment popup callbacks, iframe widget bridges, cross-window handoffs) MUST name the recipient origin for each message that carries non-public data. `/{project}:analyze` flags client-messaging specs that send such messages with a `'*'` target origin or that omit the target origin. This rule governs the sending side: a listener that acts on a message without validating its origin or its data is `FE-XSS-007`'s.

**Source:** OWASP HTML5 Security Cheat Sheet, MDN postMessage documentation

## FE-CSRF — Cross-Site Request Forgery Prevention

### FE-CSRF-001

> Every state-changing request authenticated by a credential the browser attaches automatically (session cookie, HTTP Basic, TLS client certificate) MUST be protected by at least one of: a synchronizer token or signed double-submit cookie token, transmitted in a request body field or custom header and validated server-side; a required custom request header on requests the client sends via `fetch`/`XMLHttpRequest`, under a CORS policy that does not admit that header from other origins; or server-side `Sec-Fetch-Site` validation, under which a state-changing request that lacks the header is either rejected (failing closed) or checked against its `Origin` as the fallback. A design that falls back to a CSRF token instead complies through the token defense. `SameSite` cookies and `Origin`/`Referer` checks MUST NOT be the sole defense.

**Verification:** Any spec or plan that introduces state-changing endpoints authenticated by a cookie or other ambient credential MUST name the CSRF defense the Statement requires. `/{project}:analyze` flags state-changing endpoint specs that omit the CSRF question, that name `SameSite` or `Origin`/`Referer` checks as the only defense, that propose an unsigned double-submit cookie, that carry the token the server compares only in a cookie, or anywhere in the URL, or that validate `Sec-Fetch-Site` without stating what happens to a request that lacks it. The OWASP CSRF Prevention Cheat Sheet lets `SameSite` stand alone in narrow deployments that meet five conditions, one of them an `Origin` or `Referer` check anyway; this rule does not adopt that exception. **Not flagged:** where `security-backend.md` is also loaded, a missing or inadequate CSRF defense, the token's transport included, which its `BE-API-011` owns; and, where `security-backend.md` is also loaded, a WebSocket endpoint that serves non-public data or accepts state-changing messages, its cross-site defense and a token in its opening-handshake URL included, which its `BE-API-015` owns.

**Source:** OWASP Cross-Site Request Forgery Prevention Cheat Sheet, W3C Fetch Metadata Request Headers

### FE-CSRF-002

> Session and authentication cookies MUST set the `SameSite` attribute to `Lax` or `Strict`. `SameSite=None` MUST only be used when cross-site cookie transmission is explicitly required and, when used, MUST be combined with `Secure`. Use of `SameSite=None` MUST be justified in the spec.

**Verification:** Any spec or plan that introduces session cookies or auth cookies MUST name the `SameSite` value and, if `None`, MUST justify the cross-site requirement and confirm the CSRF defense `FE-CSRF-001` requires. `/{project}:analyze` flags specs that issue a session or authentication cookie without a `SameSite` attribute, or with `SameSite=None` and no justification. **Not flagged:** the CSRF defense a `SameSite=None` cookie needs, which `FE-CSRF-001` owns or, where `security-backend.md` is also loaded, its `BE-API-011`; a missing `Secure`, which `FE-STORAGE-002` owns or, where `security-backend.md` is also loaded, its `BE-AUTHN-014`; and, where `security-backend.md` is also loaded, a missing, wrong or unjustified `SameSite` value, which its `BE-AUTHN-014` owns.

**Source:** OWASP CSRF Prevention Cheat Sheet, OWASP Session Management Cheat Sheet, MDN Set-Cookie documentation

### FE-CSRF-003

> State-changing operations MUST NOT be exposed via HTTP `GET`; `GET` requests MUST be restricted to safe, idempotent reads.

**Verification:** Any spec or plan that introduces an endpoint MUST name the HTTP method per operation. `/{project}:analyze` flags endpoint specs whose `GET` handlers are described as creating, updating, or deleting state, or that conflate read and write under a single method. **Not flagged:** an OAuth client's redirection endpoint, such as `GET /auth/callback` exchanging the code and creating the session, to which an authorization server delivers its response in the query component (RFC 6749 §4.1.2), whose cross-site defence, its `state` check, `security-cross.md` `SEC-AUTHN-002` (a) owns.

**Source:** OWASP Cross-Site Request Forgery Prevention Cheat Sheet, RFC 6749 §4.1.2

### FE-CSRF-004

> Attacker-controllable input — the URL (path, query string, or fragment), `window.name`, `document.referrer`, or `postMessage` data — MUST NOT determine the endpoint or HTTP method of a state-changing request that client-side code sends, other than by selecting from a fixed set of requests the application defines; a value from such input that becomes a parameter of the request, including a resource ID in the path, MUST be validated against the format and the set of values the application expects before the request is sent.

**Verification:** Any spec or plan that describes client-side code issuing state-changing requests whose endpoint, method, or parameters derive from the Statement's attacker-controllable input (deep links, hash-based routing, OAuth or payment return handlers, embedded widgets) MUST name how that input is constrained — a predefined request table the input selects from, or per-value validation. `/{project}:analyze` flags client-routing and deep-link specs that build a state-changing request's endpoint or method from such input, or that pass such input into a state-changing request without naming its validation.

**Source:** OWASP Cross-Site Request Forgery Prevention Cheat Sheet (Dealing with Client-Side CSRF Attacks)

## FE-STORAGE — Secure Client-Side Storage

### FE-STORAGE-001

> Secrets, API tokens, session tokens, refresh tokens, and credentials MUST NOT be stored in `localStorage`, `sessionStorage`, or IndexedDB; sensitive long-lived state MUST be held in `HttpOnly` cookies or in memory for the lifetime of the page.

**Verification:** Any spec or plan that describes client-side state, token handling, or session management MUST name the storage location for each sensitive value. `/{project}:analyze` flags client-state specs that propose persisting tokens, secrets, or credentials in browser storage APIs, including in encrypted form.

**Source:** OWASP HTML5 Security Cheat Sheet, OWASP Session Management Cheat Sheet

### FE-STORAGE-002

> Session and authentication cookies MUST set `HttpOnly`, `Secure`, and `SameSite=Lax` or `Strict`; `SameSite=None` MAY replace `Lax`/`Strict` only under the conditions `FE-CSRF-002` sets.

**Verification:** Any spec or plan that introduces a session or authentication cookie MUST commit to setting all three required attributes. `/{project}:analyze` flags specs that issue a session or authentication cookie without `HttpOnly` or `Secure`. **Not flagged:** a missing, wrong or unjustified `SameSite` value, which `FE-CSRF-002` owns or, where `security-backend.md` is also loaded, its `BE-AUTHN-014`; and, where `security-backend.md` is also loaded, a missing `HttpOnly` or `Secure`, which its `BE-AUTHN-014` owns.

**Source:** OWASP Session Management Cheat Sheet

### FE-STORAGE-003

> Sensitive data (tokens, credentials, PII, session identifiers) MUST NOT appear in URL query parameters, fragment identifiers, or path segments. A single-use, short-lived token whose flow requires URL delivery — an OAuth authorization code, an emailed password-reset or email-verification link — MAY appear in the URL, in a query parameter or a path segment, when the server invalidates it on first use and a landing page the project serves sets `Referrer-Policy: no-referrer`. A pre-signed object-storage URL MAY carry its authorization in the URL when it is scoped to one object and one method and expires within a lifetime the spec states. Neither carve-out reaches a WebSocket opening handshake, a single-use ticket included. An OpenID Provider the project serves MAY receive `login_hint` and `id_token_hint` in the query of a GET request to its authorization endpoint, and `id_token_hint` and `logout_hint` in one to its logout endpoint, and a relying party the project serves MAY receive `login_hint` in one to its login-initiation endpoint, as OpenID Connect has them accept these by GET, and a key its issuer designs to be public MAY appear in a URL a third party's browser API, embed or image takes it in, as that API requires. The hint MAYs reach only the receiver: a party that sends such a hint can POST it. The `iss` and `sid` an OpenID Provider adds to the query of a relying party's `frontchannel_logout_uri` when it renders that URI as an `iframe`'s source MAY appear there, for the provider that renders the request and the relying party that receives it, as OpenID Connect Front-Channel Logout 1.0 delivers them by no other means, and the opaque identifier an RFC 8058 one-click `List-Unsubscribe` URI carries MAY appear in that URI, which encodes the recipient and the list with no other carrier.

**Verification:** Any spec or plan that describes how the client transmits sensitive values to the server, or receives them in a URL, MUST commit to message bodies or headers, not URL components, for each such value outside the Statement's carve-outs and MAYs. `/{project}:analyze` flags client-server specs that bind tokens, credentials, session IDs, or PII into URL paths, query strings, or fragments outside the two carve-outs, neither of which reaches a WebSocket opening handshake, a single-use ticket included, other than a key its issuer designs to be public in a URL a third party's browser API, embed or image takes it in, the spec naming that API, and, in the spec of an OpenID Provider the project serves, the `login_hint` and `id_token_hint` it receives in a GET request to its authorization endpoint and the `id_token_hint` and `logout_hint` it receives in one to its logout endpoint, and, in the spec of a relying party the project serves, the `login_hint` it receives in one to its login-initiation endpoint and the `iss` and `sid` it receives at its `frontchannel_logout_uri`, client code that binds any such hint into a URL staying flagged, and other than the `iss` and `sid` an OpenID Provider the project serves renders into that URI and the opaque identifier of an RFC 8058 one-click `List-Unsubscribe` URI, flags a URL-delivered single-use token whose spec omits, for a token the project issues, its first-use invalidation or expiry, or, on a landing page the project serves, `Referrer-Policy: no-referrer`, the callback page of a client another party operates not being one, and flags a pre-signed URL the project issues whose spec states no expiry or whose scope reaches more than one object or method. **Not flagged:** the first-use invalidation and expiry of a single-use value, and the expiry and scope of a pre-signed URL, that a third party issues, such as an authorization code a client receives from another party's authorization server; a sensitive configuration value, alone or in a configuration object, that the project's application returns to a client in a URL a response carries, a redirect's `Location` or a link in a page it renders included, or sends to a person in a link in an email or another message, or that client code takes from a response the application returns and binds into the URL of an endpoint the project does not serve, in a spec or plan that has the application return the value, which `configuration-cross.md` `CFG-ENV-006` owns, and one the build injects into an artifact delivered to end users, bound into a URL by client code included, such as a `VITE_GEOCODE_KEY` placed in a `?key=` query, which its `CFG-ENV-010` owns, a pre-signed URL the project issues and client code binding one the build did not inject into the URL of such an endpoint in a spec or plan that does not have the application return the value staying this rule's; where `security-backend.md` is also loaded, a password, session identifier, access, refresh or ID token, API key, or PII in a query parameter or path segment of the URL of an endpoint the project serves, or any credential sent to one with no carrier named, whether the spec describes the server accepting it, the client sending it or the server sending it to a client, which its `BE-API-013` owns or, for the carrier of a WebSocket endpoint that serves non-public data or accepts state-changing messages in a spec or plan that introduces one, its `BE-API-015` (b); and, where `security-backend.md` is also loaded, any password, session identifier, token or API key, a single-use ticket included, in the opening-handshake URL of a WebSocket endpoint the project serves that serves non-public data or accepts state-changing messages, which its `BE-API-015` owns; and, where `security-backend.md` is also loaded, a password-reset or account-recovery token's expiry and first-use invalidation, which its `BE-AUTHN-018` owns; and any value in a URL fragment that an OAuth authorization response returns under a response type that returns an access token there, the implicit grant's `response_type=token` among them, in a spec or plan that introduces an OAuth client or authorization server or changes the grant or response types one uses or supports, which `security-cross.md` `SEC-AUTHN-003` owns; and a CSRF token's transport, in a spec or plan that introduces the state-changing endpoint, which `FE-CSRF-001` owns or, where `security-backend.md` is also loaded, its `BE-API-011`, outside a handshake that, where `security-backend.md` is also loaded, its `BE-API-015` owns, a spec or plan that describes a form or client code sending one and introduces none staying this rule's. The other conditions of this rule's two carve-outs, a reset landing page's `Referrer-Policy: no-referrer` included, stay this rule's outside a handshake that, where `security-backend.md` is also loaded, its `BE-API-015` owns.

**Source:** OWASP REST Security Cheat Sheet, OWASP Session Management Cheat Sheet, OWASP Forgot Password Cheat Sheet, RFC 9700 (OAuth 2.0 Security Best Current Practice), MDN Referer and URI fragment documentation, Amazon S3 User Guide (Download and upload objects with presigned URLs), OpenID Connect Core 1.0 §3.1.2.1 and §4, OpenID Connect RP-Initiated Logout 1.0 §2, OpenID Connect Front-Channel Logout 1.0 §2, RFC 8058 §3.1, Google Maps Platform, "Load the Maps JavaScript API", "The Maps Embed API overview" and "Google Maps Platform security guidance"

### FE-STORAGE-004

> The `Domain` attribute SHOULD NOT be set on session and authentication cookies unless cross-subdomain sharing is explicitly required.

**Verification:** Any spec or plan that introduces a session or authentication cookie SHOULD leave `Domain` unset, or justify cross-subdomain sharing where it sets one. `/{project}:analyze` emits a warning when a spec sets `Domain` on a session or authentication cookie without justification.

**Source:** OWASP Session Management Cheat Sheet

### FE-STORAGE-005

> Session and authentication cookies SHOULD use the `__Host-` name prefix.

**Verification:** Any spec or plan that introduces a session or authentication cookie SHOULD name it with the `__Host-` prefix, or state why the cookie needs a `Domain` attribute or a narrower `Path`. `/{project}:analyze` emits a warning when a cookie-issuing spec names a session or authentication cookie without the `__Host-` prefix and without that justification.

**Source:** OWASP Session Management Cheat Sheet, OWASP Cross-Site Request Forgery Prevention Cheat Sheet, MDN Set-Cookie documentation (cookie prefixes)

### FE-STORAGE-006

> Service worker code MUST NOT store responses that carry sensitive data — tokens, credentials, PII, or other user-specific authenticated content — in the Cache API.

**Verification:** Any spec or plan that introduces or changes a service worker that caches responses (offline support, precaching, runtime caching strategies) MUST name which routes or response types it caches and commit to excluding authenticated and sensitive responses. `/{project}:analyze` flags service-worker specs that apply a caching strategy to all routes or to authenticated API responses without naming that exclusion, or that rely on `Cache-Control: no-store` to keep responses out of the Cache API.

**Source:** OWASP HTML5 Security Cheat Sheet (Offline Applications), MDN Cache documentation

## FE-AUTHN — Authentication UX

### FE-AUTHN-001

> Post-authentication redirect destinations MUST be validated against an allowlist of internal application paths or hosts; open redirects (any user-controlled redirect target accepted without validation) MUST NOT be permitted.

**Verification:** Any spec or plan that describes post-login redirects, "return to" URLs, OAuth callback handling, or any feature that accepts a destination URL from request parameters or otherwise redirects to a user-controlled destination MUST name the allowlist (paths or hosts). `/{project}:analyze` flags login or OAuth flows that pass through a `next`/`return_to`/`redirect_uri` value without naming an allowlist. **Not flagged:** an OAuth or OIDC client's `redirect_uri`, in a spec or plan that introduces the client, adds or changes its redirection endpoint, or changes the redirect URIs it registers or sends, which `security-cross.md` `SEC-AUTHN-002` (d) owns; where `security-backend.md` is also loaded, a redirect the server's own response performs, a `3xx` response's `Location`, a meta refresh or script into which the server writes the destination to perform the redirect, or a server action's redirect, which its `BE-API-009` owns; a destination client-side code reads and navigates to, whatever its source (the URL, `document.referrer`, `window.name`, storage, a message, a fetched response, the project's own API's included, or page data or props the server renders) stays this rule's, an authorization server's `redirect_uri` below aside; and a `redirect_uri` an OAuth authorization server the project serves or provides receives, its matching and client-side code's navigation to it included, in a spec or plan that introduces or describes that server, which `security-cross.md` `SEC-AUTHN-004` owns.

**Source:** OWASP Unvalidated Redirects and Forwards Cheat Sheet

### FE-AUTHN-002

> When the user's session expires or is revoked, the application MUST surface a clear re-authentication prompt or redirect to the login page; the application MUST NOT silently fail, retry indefinitely, or leave the UI in a broken state.

**Verification:** Any spec or plan that describes authenticated UI flows MUST name the session-expiration UX (redirect, modal prompt, banner with re-auth). `/{project}:analyze` flags authenticated-UI specs that omit session-expiration handling or that describe error states (e.g., "request fails") without naming the recovery path.

**Source:** OWASP Session Management Cheat Sheet

### FE-AUTHN-003

> The logout action MUST invalidate the session on the server, clear all client-side session cookies, and redirect to a public page; client-side-only logout (clearing cookies without server invalidation) MUST NOT be used.

**Verification:** Any spec or plan that introduces logout, session termination, or "sign out everywhere" flows MUST commit to server-side session invalidation and explicit cookie clearing. `/{project}:analyze` flags logout specs that describe only clearing cookies, calling `localStorage.clear`, or redirecting without naming the server invalidation step.

**Source:** OWASP Session Management Cheat Sheet

## FE-CSP — Content Security Policy

### FE-CSP-001

> All HTML responses MUST be served with a `Content-Security-Policy` HTTP header; CSP MUST be delivered via the response header.

**Verification:** Any spec or plan that describes serving HTML responses or configuring the edge/web server MUST commit to a CSP header on every HTML response. `/{project}:analyze` flags HTML-serving specs that omit a CSP commitment or that describe `<meta>`-only CSP delivery.

**Source:** W3C Content Security Policy Level 3, OWASP CSP Cheat Sheet, MDN CSP documentation

### FE-CSP-002

> The CSP policy MUST use nonce-based or hash-based script restrictions. The policy MUST NOT include `'unsafe-inline'` or `'unsafe-eval'` in `script-src`, MUST NOT include `'unsafe-inline'` in `style-src` (nonce/hash or `'self'` only), and MUST include `object-src 'none'` and `base-uri 'none'` (or `'self'`).

**Verification:** Any spec or plan that defines a CSP policy MUST name the nonce/hash strategy for scripts and a `style-src` strategy the Statement allows and MUST commit to the Statement's exclusions and its `object-src` and `base-uri` lockdown. `/{project}:analyze` flags CSP specs that include a keyword the Statement excludes — even beside a nonce or hash, as a fallback for older browsers — or that omit `object-src` or `base-uri`.

**Source:** OWASP CSP Cheat Sheet, W3C Content Security Policy Level 3, MDN CSP documentation, XS-Leaks Wiki (CSS Injection)

### FE-CSP-003

> The CSP policy MUST include `frame-ancestors 'none'` (or `'self'` if same-origin framing is required). A page designed to be embedded by other sites MAY instead list the exact origins permitted to frame it; wildcard sources (`*`, `https:`) MUST NOT be used.

**Verification:** Any spec or plan that describes serving HTML responses MUST commit to a `frame-ancestors` value the Statement allows. `/{project}:analyze` flags HTML-serving specs that omit `frame-ancestors`, which does not fall back to `default-src`, or that give it a wildcard source.

**Source:** OWASP HTTP Headers Cheat Sheet, OWASP Clickjacking Defense Cheat Sheet, W3C Content Security Policy Level 3, MDN CSP `frame-ancestors` documentation

### FE-CSP-004

> The CSP policy SHOULD include `form-action 'self'` (or an explicit destination allowlist).

**Verification:** Any spec or plan that defines a CSP policy SHOULD commit to a `form-action` value the Statement allows. `/{project}:analyze` emits a warning when CSP specs omit `form-action`, which does not fall back to `default-src`.

**Source:** OWASP CSP Cheat Sheet, MDN CSP `form-action` documentation

### FE-CSP-005

> The CSP policy MUST include a reporting directive — `report-to` (preferred) with a matching `Reporting-Endpoints` header, and/or `report-uri` — pointing to an endpoint that ingests CSP violation reports. The endpoint MUST keep the reports where the team responsible for the CSP can review them, not discard them on receipt.

**Verification:** Any spec or plan that defines a CSP policy MUST commit to a reporting directive and to an ingest path that keeps the reports reviewable by the responsible team. `/{project}:analyze` flags CSP specs that omit both `report-to` and `report-uri`, and reporting-endpoint specs that describe discarding reports on receipt (to `/dev/null`, for example) or that name no place the reports are kept. **Not flagged:** how the team watches the kept reports, which is its own operational choice.

**Source:** W3C Content Security Policy Level 3, OWASP CSP Cheat Sheet, MDN Reporting API

### FE-CSP-006

> Iframes embedding content the application does not fully control (third-party widgets, social-media embeds, user-supplied URLs, payment forms hosted elsewhere) MUST set the `sandbox` attribute with the minimum capabilities required for the embed to function. `sandbox=""` (full restrictions) is the floor; each token added (`allow-scripts`, `allow-same-origin`, `allow-forms`, etc.) MUST be justified. `allow-scripts` plus `allow-same-origin` together MUST NOT be used on a frame whose document is served from the embedding page's own origin (user-authored or user-uploaded HTML, for example).

**Verification:** Any spec or plan that introduces iframes embedding non-fully-controlled content MUST commit to a `sandbox` value and MUST justify each token included. `/{project}:analyze` flags iframe specs that omit `sandbox`, that use it without naming the included tokens, or that combine `allow-scripts` with `allow-same-origin` on a frame whose content is served from the embedding page's origin. **Not flagged:** `allow-scripts` with `allow-same-origin`, each justified, on a frame served from another origin.

**Source:** OWASP HTML5 Security Cheat Sheet, MDN iframe sandbox documentation

### FE-CSP-007

> HTML responses MUST set the `Permissions-Policy` header (formerly `Feature-Policy`) restricting access to powerful browser features the application does not use. At minimum, features with privacy or security impact MUST be explicitly denied unless the application genuinely needs them: `camera=()`, `microphone=()`, `geolocation=()`, `payment=()`, `usb=()`. Features the application uses MUST be scoped to `self` (or an explicit origin allowlist) rather than left open.

**Verification:** Any spec or plan that describes serving HTML responses MUST commit to a `Permissions-Policy` header naming the denied and allowed features. `/{project}:analyze` flags HTML-serving specs that omit `Permissions-Policy`, that leave privacy-sensitive features unrestricted, or that grant features without an origin scope. **Not flagged:** a policy without `interest-cohort=()`, a privacy opt-out, not a powerful feature a script can prompt the user for.

**Source:** W3C Permissions Policy specification, OWASP HTTP Headers Cheat Sheet, MDN Permissions-Policy documentation, Topics API explainer

### FE-CSP-008

> Legacy `X-Frame-Options: DENY` (or `SAMEORIGIN` where same-origin framing is required) SHOULD also be set as a clickjacking fallback for browsers that do not honor `frame-ancestors`.

**Verification:** Any spec or plan that describes serving HTML responses SHOULD commit to `X-Frame-Options: DENY` (or `SAMEORIGIN`). `/{project}:analyze` emits a warning when HTML-serving specs omit `X-Frame-Options`. `BE-API-012` is this rule's backend mirror, for a backend that serves HTML without loading this file.

**Source:** OWASP HTTP Headers Cheat Sheet, OWASP Clickjacking Defense Cheat Sheet

### FE-CSP-009

> HTML responses SHOULD set `Cross-Origin-Opener-Policy: same-origin`, or `same-origin-allow-popups` on a page that opens cross-origin popups which must keep their reference to it (OAuth or payment popups that report back through `window.opener`).

**Verification:** Any spec or plan that describes serving HTML responses SHOULD commit to a `Cross-Origin-Opener-Policy` value, naming the popup flow wherever it chooses `same-origin-allow-popups` over `same-origin`. `/{project}:analyze` emits a warning when HTML-serving specs omit `Cross-Origin-Opener-Policy` or set it to `unsafe-none` without justification. **Not flagged:** a missing `Cross-Origin-Embedder-Policy` or full cross-origin isolation, which are outside this rule.

**Source:** OWASP HTTP Headers Cheat Sheet, MDN Cross-Origin-Opener-Policy documentation, XS-Leaks Wiki

## FE-DEPS — Dependency Management

### FE-DEPS-001

> Frontend dependencies MUST be scanned for known vulnerabilities before every release and on a recurring schedule between releases; dependencies with known critical or high-severity vulnerabilities MUST be updated, replaced, or accompanied by a documented exception with a remediation date.

**Verification:** Any spec or plan covering frontend dependency management, a frontend build or its deployment, or `system.md` where it describes the frontend build MUST name the vulnerability scanner (Snyk, Dependabot, `npm audit`, `pnpm audit`, `yarn npm audit` or Yarn 1's `yarn audit`, OWASP Dependency-Check), when it runs (before release and on what schedule), AND the policy on critical/high findings (block the release, file an issue, alert on-call). `/{project}:analyze` flags such specs that omit any of the three commitments for the frontend's dependencies. **Not flagged:** how the scan runs — a CI job, a scheduled job, a dependency bot — which is the project's choice; and a dependency change not scanned when it is made, which `FE-DEPS-006` owns.

**Source:** OWASP Dependency-Check, OWASP Top 10:2025 (A03:2025 — Software Supply Chain Failures, an expansion of A06:2021 — Vulnerable and Outdated Components)

### FE-DEPS-002

> Third-party scripts and stylesheets loaded from cross-origin CDNs MUST include **both** the `integrity` attribute (with a valid Subresource Integrity hash) **and** the `crossorigin` attribute (typically `crossorigin="anonymous"`). A cross-origin script referenced from markup or build output without integrity verification MUST NOT be loaded; scripts injected at runtime are governed by `FE-DEPS-003`.

**Verification:** Any spec or plan that describes loading scripts, stylesheets, or modules from external origins MUST commit to SRI **plus** `crossorigin` on every cross-origin asset and MUST commit to refusing to load assets without an integrity check. `/{project}:analyze` flags HTML-template or build-output specs that name third-party CDNs, externally hosted scripts, or remotely loaded modules without naming both attributes.

**Source:** MDN Subresource Integrity documentation, W3C Subresource Integrity specification, OWASP CSP Cheat Sheet

### FE-DEPS-003

> The application MUST NOT dynamically inject a script from a third-party origin at runtime (ad-hoc `document.createElement('script')` or equivalent pointing at an external URL) unless the load carries integrity verification or is gated by a CSP-and-vendor-review process.

**Verification:** Any spec or plan that describes runtime third-party integrations (analytics, A/B testing, support widgets, embed providers) MUST commit to bundling the third-party code with build-time SRI, to integrity verification on the runtime-injected script, or to a CSP-and-vendor-review process for the runtime load. `/{project}:analyze` flags integration specs that describe creating script elements pointing to external URLs without naming an integrity or CSP-trust commitment.

**Source:** OWASP CSP Cheat Sheet

### FE-DEPS-004

> Production frontend dependencies MUST be pinned to specific versions (or version + integrity hash) in the lockfile used for production builds; floating versions (`latest`, `^`, `~`, range constraints) MUST NOT determine what ships to production. A pinned version MUST change only through a documented upgrade process.

**Verification:** Any spec or plan covering a frontend production build, its deployment, or `system.md` where it describes one MUST commit to (a) an authoritative lockfile (`package-lock.json`, `pnpm-lock.yaml`, `yarn.lock`) that the production build installs from and (b) a documented upgrade process. `/{project}:analyze` flags such specs that allow floating dependency ranges in the frontend production build, that omit the lockfile question, or that describe a frontend production install able to rewrite the lockfile: `npm install` rather than `npm ci`, `pnpm install` without `--frozen-lockfile`, or a Yarn install without `--immutable` (Yarn 1: `--frozen-lockfile`), except where pnpm or Yarn 3+ already applies that behavior by default because it detects a CI environment. **Not flagged:** a package a build step's command runs by a mutable reference, such as `npx tool@latest`, which `security-cross.md` `SEC-DEPS-001` owns.

**Source:** OWASP Top 10:2025 (A03:2025 — Software Supply Chain Failures), npm `ci` documentation, pnpm `install` documentation, Yarn `install` documentation

### FE-DEPS-005

> A frontend dependency MUST NOT originate network requests to third-party origins as a side effect of being used. Telemetry, usage analytics, license checks, and update polling performed by a dependency MUST be disabled at build time through the opt-out the dependency provides, and the opt-out MUST apply to every build configuration the team runs, not only to production builds. An intentional third-party integration is exempt only when it is a declared part of the application's design, permitted explicitly by the CSP `connect-src` directive, and vendor reviewed. Verification that no unintended egress occurs MUST be performed against a production build.

**Verification:** Any spec or plan that introduces or upgrades a frontend dependency shipping to the browser MUST state whether that dependency performs network egress of its own and, where it does, name the documented opt-out and the build configurations the opt-out covers. `/{project}:analyze` flags specs adopting UI component libraries, analytics-capable SDKs, error/crash reporters, font or icon providers, and map or media embeds without addressing dependency egress, and flags a verification commitment that names only a development-server check. This rule governs what a dependency does on the network once it runs: what code is loaded, and from where, is the other `FE-DEPS` rules'.

**Source:** OWASP Top 10:2025 (A03:2025 — Software Supply Chain Failures); OWASP Top 10 Client-Side Security Risks (candidate list: Sensitive Data Leakage, Lack of Third-party Origin Control)

### FE-DEPS-006

> The vulnerability scan of `FE-DEPS-001` SHOULD also run automatically on every change that alters the frontend dependency set — the manifest or the lockfile.

**Verification:** Any spec or plan covering frontend dependency management or the frontend's CI configuration SHOULD commit to an automated scan on changes to the frontend dependency set. `/{project}:analyze` emits a warning when such a spec omits it. **Not flagged:** where the scan runs — CI, a pre-commit hook, a dependency bot — which is the project's choice.

**Source:** OWASP Dependency-Check, OWASP Top 10:2025 (A03:2025 — Software Supply Chain Failures)

### FE-DEPS-007

> Every production frontend build artifact MUST be accompanied by a machine-readable Software Bill of Materials (SBOM) in CycloneDX or SPDX format, generated during the build (not after) and retained for the lifetime of the artifact.

**Verification:** Any spec or plan covering a frontend production build, its release, or `system.md` where it describes one MUST name the SBOM tool and format and commit to per-build generation and retention. `/{project}:analyze` flags such specs that omit SBOM generation or that describe one-off SBOM exports separate from the build. **Not flagged:** where `security-backend.md` is also loaded, the SBOM of any production build artifact, the frontend's included, which its `BE-DEPS-003` owns.

**Source:** OWASP Dependency Graph & SBOM Cheat Sheet, CycloneDX / SPDX specifications

### FE-DEPS-008

> Frontend dependencies pulled from a public registry MUST be verified against the registry's signatures and any provenance attestations the packages carry as part of installation, before they are built into a production artifact, such as with `npm audit signatures`, which checks both. A package that fails verification MUST be replaced, vendored, or accompanied by a documented exception with a remediation plan.

**Verification:** Any spec or plan covering frontend dependency management, a frontend production build, or `system.md` where it describes one MUST commit to the verification step in the install. A lockfile integrity hash, which `FE-DEPS-009` (b) requires, is not such a step. `/{project}:analyze` flags such specs that install from a public registry without naming a verification step. **Not flagged:** where `security-backend.md` is also loaded, the verification of any dependency, the frontend's included, which its `BE-DEPS-004` owns.

**Source:** npm documentation — `npm audit` (Audit Signatures) and Generating provenance statements

### FE-DEPS-009

> Frontend builds MUST be configured to prevent dependency-confusion and typosquatting attacks: (a) when private packages share a namespace with a public registry, a private package name MUST resolve only from the private registry, by an npm scope associated with it (`@myco:registry` in `.npmrc`); (b) the production install MUST verify each package against the integrity hash its lockfile commits (`package-lock.json`'s `integrity`); (c) new dependency additions MUST be reviewed against a known-typo list for the project's hot dependencies.

**Verification:** Any spec or plan covering frontend dependency management or a frontend build configuration MUST commit to the Statement's (a) when private packages exist, (b), and (c). `/{project}:analyze` flags such specs that omit any of these when the project uses private packages or has a high dependency-churn rate, and flags a private package name the install can resolve from the public registry. **Not flagged:** where `security-backend.md` is also loaded, any build's dependency-confusion and typosquatting defence, the frontend's included, which its `BE-DEPS-005` owns.

**Source:** OWASP Software Supply Chain Security Cheat Sheet (Threat landscape: dependency confusion); npm documentation — scope and package-lock.json (`integrity`)

## FE-PII — Sensitive Data Handling

### FE-PII-001

> When personally identifiable information is rendered in the UI, the value MUST be masked or partially redacted unless the full value is required for the user's task; reveal-on-demand interactions (click-to-reveal, hover-with-delay) MUST require an explicit user action.

**Verification:** Any spec or plan that introduces UI surfaces displaying PII, financial data, or government IDs MUST name the masking strategy (which characters are visible, which are redacted) and any reveal mechanism. `/{project}:analyze` flags UI specs that render full PII fields without a masking commitment or reveal-on-demand interaction.

**Source:** PCI DSS v4.0.1 Requirement 3.4.1 (PAN masked when displayed), NIST SP 800-122 section 4.2.1 (minimizing the use of PII)

### FE-PII-002

> Forms collecting credentials, multi-factor codes, and payment data MUST set the appropriate `autocomplete` token (`current-password`, `new-password`, `cc-number`, `one-time-code`, etc.); blanket `autocomplete="off"` on credential and payment fields MUST NOT be used as the default posture.

**Verification:** Any spec or plan that introduces credential, MFA, or payment forms MUST name the `autocomplete` token per field. `/{project}:analyze` flags form specs that set `autocomplete="off"` blanketly across credential or payment inputs without justification, and flags credential forms that omit the `autocomplete` question entirely.

**Source:** OWASP Authentication Cheat Sheet (Password Managers), MDN HTML attribute: autocomplete, NIST SP 800-63B-4 section 3.1.1.2

### FE-PII-003

> Pages displaying sensitive data MUST set `Cache-Control: no-store`.

**Verification:** Any spec or plan that describes pages displaying sensitive data MUST commit to `Cache-Control: no-store` on them. `/{project}:analyze` flags specs describing such a page that omit `Cache-Control: no-store`. **Not flagged:** a sensitive response a service worker stores in the Cache API, which `FE-STORAGE-006` owns; and, where `security-backend.md` is also loaded, a missing `Cache-Control: no-store` on a page whose response carries sensitive data, or on an authenticated or sensitive-data response, which its `BE-API-001` owns.

**Source:** OWASP Session Management Cheat Sheet, MDN Cache-Control documentation

### FE-PII-004

> The logout response SHOULD include `Clear-Site-Data: "cache", "cookies", "storage"`. A deployment that shares its registrable domain with other applications SHOULD either name the effect of `"cookies"` on them or omit `"cookies"` and rely on server-side invalidation (`FE-AUTHN-003`).

**Verification:** Any spec or plan that describes logout flows SHOULD commit to `Clear-Site-Data` on the logout response. `/{project}:analyze` emits a warning when logout flows omit `Clear-Site-Data`, or when a deployment sharing its domain with other applications sends `"cookies"` without naming its effect on them. The `"cookies"` directive clears cookies for the entire registrable domain, subdomains included, so a logout on one subdomain also ends sessions held by sibling applications on that domain.

**Source:** OWASP Session Management Cheat Sheet, MDN Clear-Site-Data documentation, W3C Clear Site Data specification
